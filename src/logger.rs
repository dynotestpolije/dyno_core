// TODO: include the changelog as a module when
// https://github.com/rust-lang/rust/issues/44732 stabilises

use log::{Level, LevelFilter, Log, Metadata, Record};
use std::io;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;

use crate::DynoResult;

lazy_static::lazy_static! {
    pub static ref RECORDS_LOGGER: Mutex<Vec<(Level, String)>> = Default::default();
    static ref LOGGER: DynoLogger = DynoLogger::default();
}

#[derive(Default)]
pub struct DynoLogger {
    inner: Mutex<Option<DynoLoggerInner>>,
}

impl DynoLogger {
    // Set this `SimpleLogger`'s sink and reset the start time.
    #[allow(unused)]
    fn renew<T: Write + Send + 'static>(&self, sink: T) -> DynoResult<'_, ()> {
        let mut lock = self
            .inner
            .lock()
            .map_err(|_| "Failed to lock the mutex Logger")?;
        *lock = Some(DynoLoggerInner {
            sink: Box::new(sink),
        });

        Ok(())
    }
}

impl Log for DynoLogger {
    #[inline(always)]
    fn enabled(&self, m: &Metadata) -> bool {
        m.level() < log::max_level()
    }

    #[inline]
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        if let Ok(mut locked) = self.inner.lock() {
            if let Some(ref mut inner) = *locked {
                inner.log(record);
            }
        }
    }

    #[inline]
    fn flush(&self) {
        if let Ok(mut locked) = self.inner.lock() {
            if let Some(ref mut inner) = *locked {
                inner.flush()
            }
        }
    }
}

struct DynoLoggerInner {
    sink: Box<dyn Write + Send>,
}

impl DynoLoggerInner {
    fn log(&mut self, record: &Record) {
        let level = record.level();
        let record_string = format!(
            "[{}]:[{}]:{:6} - {}\n",
            chrono::Utc::now().format("%+"),
            record.metadata().target(),
            level,
            record.args()
        );
        self.sink.write(record_string.as_bytes()).ok();

        if let Ok(mut locked) = RECORDS_LOGGER.lock() {
            locked.push((level, record_string));
        }
    }
    #[inline]
    fn flush(&mut self) {
        self.sink.flush().ok();
    }
}

#[allow(unused)]
pub fn log_to_file<'a, T: AsRef<Path>>(
    path: T,
    max_log_level: LevelFilter,
    append: bool,
) -> DynoResult<'a, ()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = std::fs::OpenOptions::new()
        .write(true)
        .append(append)
        .truncate(!append)
        .create(true)
        .open(path)?;

    LOGGER.renew(file)?;
    log::set_max_level(max_log_level);
    // The only possible error is if this has been called before
    log::set_logger(&*LOGGER).map_err(|err| {
        crate::DynoErr::new(
            format!("Failed to set logger - {err}"),
            crate::ErrKind::AnyError,
        )
    })
}

#[allow(unused)]
pub fn log_to_stderr<'a>(max_log_level: LevelFilter) -> DynoResult<'a, ()> {
    LOGGER.renew(io::stderr())?;
    log::set_max_level(max_log_level);
    // The only possible error is if this has been called before
    log::set_logger(&*LOGGER).map_err(|err| {
        crate::DynoErr::new(
            format!("Failed to set logger - {err}"),
            crate::ErrKind::AnyError,
        )
    })
}
