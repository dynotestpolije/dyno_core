// TODO: include the changelog as a module when
// https://github.com/rust-lang/rust/issues/44732 stabilises

use log::{LevelFilter, Log, Metadata, Record};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;

use crate::DynoResult;

lazy_static::lazy_static! {
    pub static ref LOGGER: DynoLogger = DynoLogger::default() ;
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
            start: Instant::now(),
            sink: Box::new(sink),
        });

        Ok(())
    }
}

impl Log for DynoLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

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

    fn flush(&self) {}
}

struct DynoLoggerInner {
    start: Instant,
    sink: Box<dyn Write + Send>,
}

impl DynoLoggerInner {
    fn log(&mut self, record: &Record) {
        let now = self.start.elapsed();
        let seconds = now.as_secs();
        let hours = seconds / 3600;
        let minutes = (seconds / 60) % 60;
        let seconds = seconds % 60;
        let miliseconds = now.subsec_millis();

        let _ = writeln!(
            self.sink,
            "[{:02}:{:02}:{:02}.{:03}] {:6} {}",
            hours,
            minutes,
            seconds,
            miliseconds,
            record.level(),
            record.args()
        )
        .ok();
    }
}

#[inline(always)]
#[allow(unused)]
pub fn log_to_file<'a, T: AsRef<Path>>(path: T, max_log_level: LevelFilter) -> DynoResult<'a, ()> {
    let file = File::create(path)?;
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
#[inline(always)]
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
