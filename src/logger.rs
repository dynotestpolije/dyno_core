// TODO: include the changelog as a module when
// https://github.com/rust-lang/rust/issues/44732 stabilises

use log::{Level, LevelFilter, Log, Metadata, Record};
use std::fs::File;
use std::io::{self, BufWriter};
use std::io::{Stderr, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

use crate::{DynoErr, DynoResult};

lazy_static::lazy_static! {
    pub static ref RECORDS_LOGGER: Mutex<Vec<(Level, String)>> = Default::default();
    static ref LOGGER: DynoLogger = DynoLogger::default();
}

trait DynoLoggerInner: Send + 'static {
    fn log(&mut self, level: Level, record: String);
    fn flush(&mut self);

    fn roll(&mut self) {}
}

pub struct DynoLogger {
    inner: Mutex<Box<dyn DynoLoggerInner>>,
}

impl Default for DynoLogger {
    fn default() -> Self {
        Self {
            inner: Mutex::new(Box::new(ConsoleLogger::new())),
        }
    }
}

impl DynoLogger {
    // Set this `SimpleLogger`'s sink and reset the start time.
    #[allow(unused)]
    fn renew(&self, inner: Box<dyn DynoLoggerInner>) -> DynoResult<()> {
        let mut lock = self
            .inner
            .lock()
            .map_err(|_| "Failed to lock the mutex Logger")?;
        *lock = inner;
        Ok(())
    }
}

impl Log for DynoLogger {
    #[inline(always)]
    fn enabled(&self, m: &Metadata) -> bool {
        log::log_enabled!(target: "Global", m.level())
    }

    #[inline]
    fn log(&self, record: &Record) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.roll();
            let level = record.level();
            let target = record.metadata().target();
            let args = record.args();
            inner.log(
                level,
                format!(
                    "[{}]::[{}]::[{:6} - {}]\n",
                    chrono::Utc::now().format("%v - %T"),
                    target,
                    level,
                    args,
                ),
            );
            if let Ok(mut locked) = RECORDS_LOGGER.lock() {
                locked.push((level, format!("[{}]::[{:6}] - {}", target, level, args)));
            }
        }
    }

    #[inline]
    fn flush(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.flush()
        }
    }
}

struct ConsoleLogger {
    sink: Stderr,
}
impl ConsoleLogger {
    fn new() -> Self {
        Self { sink: io::stderr() }
    }
}

impl DynoLoggerInner for ConsoleLogger {
    fn log(&mut self, _level: Level, record: String) {
        self.sink.lock().write(record.as_bytes()).ok();
    }

    fn flush(&mut self) {
        self.sink.flush().ok();
    }
}

fn open_file<P: AsRef<Path>>(file: P) -> DynoResult<File> {
    let file = file.as_ref();
    std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .truncate(false)
        .create(true)
        .open(file)
        .map_err(DynoErr::filesystem_error)
}

#[inline]
fn format_system_time<'a>(
    t: &SystemTime,
    fmt: &'a str,
) -> chrono::format::DelayedFormat<chrono::format::StrftimeItems<'a>> {
    let time = t
        .duration_since(std::time::UNIX_EPOCH)
        .expect("systemtime since epoch");
    chrono::NaiveDateTime::from_timestamp_opt(time.as_secs() as i64, time.subsec_nanos())
        .expect("systemtime to chrono::NaiveDateTime")
        .format(fmt)
}

struct FileLogger<W: io::Write + Send + 'static> {
    #[allow(dead_code)]
    file: PathBuf,
    sink: BufWriter<W>,
    action: FileAction,
    last_access: SystemTime,
    max_len: usize,
    len: usize,
}

impl FileLogger<File> {
    fn new(file: PathBuf, action: FileAction, max_len: usize) -> DynoResult<Self> {
        let fp = open_file(&file)?;
        let metadata = fp.metadata().map_err(DynoErr::filesystem_error)?;
        let last_access = metadata.modified().map_err(DynoErr::filesystem_error)?;

        let len = metadata.len() as usize;
        Ok(Self {
            sink: BufWriter::with_capacity(1024, fp),
            file,
            action,
            max_len,
            len,
            last_access,
        })
    }
}

impl DynoLoggerInner for FileLogger<File> {
    fn log(&mut self, _level: Level, record: String) {
        match self.sink.write(record.as_bytes()) {
            Ok(k) => self.len += k,
            Err(_err) => {}
        }
    }

    #[inline]
    fn flush(&mut self) {
        self.sink.flush().ok();
    }

    fn roll(&mut self) {
        if self.len <= self.max_len {
            return;
        }
        let rolled = match self.action {
            FileAction::Noop => return,
            FileAction::Roll => {
                let mut file = self.file.clone();
                file.set_extension(format!(
                    "log.{}.bak",
                    format_system_time(&self.last_access, "%+")
                ));
                // guaranteed self.file is exists, its ok to ignore error
                if std::fs::rename(&self.file, file).is_ok() {
                    self.len = 0;
                    true
                } else {
                    false
                }
            }
            FileAction::Delete => {
                let mut ret = true;
                let mut renamed = self.file.clone();
                renamed.set_extension("log.bak");
                if renamed.exists() {
                    ret = std::fs::remove_file(&renamed).is_ok();
                }
                std::fs::rename(&self.file, renamed).is_ok() && ret
            }
        };
        if rolled {
            if let Ok(other) = Self::new(self.file.clone(), self.action, self.max_len) {
                *self = other;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileAction {
    #[default]
    /// do nothing, keep appending log in file
    Noop,
    /// Crate new log file and move the old file with prefix `{file}`.`{last-access-time}`.bak
    Roll,
    /// delete the old file and create new file with the same name and locations
    Delete,
}

const SIZE_TRIGGER_ROLLING_FILE: usize = 10 * 1024 * 1024; // 50Mb
                                                           //
#[derive(Debug, Clone)]
pub struct LoggerBuilder {
    file: PathBuf,
    max_level: LevelFilter,
    roll_action: FileAction,
    max_size: usize,
}
impl Default for LoggerBuilder {
    fn default() -> Self {
        Self {
            file: std::env::temp_dir().join("dynotest/log.log"),
            max_level: LevelFilter::Warn,
            roll_action: FileAction::Roll,
            max_size: SIZE_TRIGGER_ROLLING_FILE,
        }
    }
}

#[allow(unused)]
impl LoggerBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_file(mut self, file: impl Into<PathBuf>) -> Self {
        self.file = file.into();
        self
    }
    pub fn set_max_level(mut self, level: impl Into<LevelFilter>) -> Self {
        self.max_level = level.into();
        self
    }
    pub fn set_roll_action(mut self, action: FileAction) -> Self {
        self.roll_action = action;
        self
    }
    pub fn set_max_size(mut self, max_in_mb: usize) -> Self {
        self.max_size = max_in_mb * 1024 * 1024;
        self
    }

    pub fn build_console_logger(self) -> DynoResult<()> {
        LOGGER.renew(Box::new(ConsoleLogger::new()))?;

        log::set_max_level(self.max_level);
        log::set_logger(&*LOGGER).map_err(DynoErr::logger_error)
    }

    pub fn build_file_logger(self) -> DynoResult<()> {
        if let Some(parent) = self.file.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let file_logger = FileLogger::new(self.file, self.roll_action, self.max_size)?;
        LOGGER.renew(Box::new(file_logger));
        log::set_max_level(self.max_level);
        log::set_logger(&*LOGGER).map_err(DynoErr::logger_error)
    }
}
