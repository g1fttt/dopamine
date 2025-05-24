use chrono::Local as LocalChrono;
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{Mutex, OnceLock};

static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn init<P>(path: P) -> Result<(), SetLoggerError>
where
  P: AsRef<Path> + Display,
{
  if LOGGER.get().is_some() || !cfg!(debug_assertions) {
    return Ok(());
  }

  let _ = std::fs::create_dir(Logger::PATH);

  log::set_logger(LOGGER.get_or_init(|| {
    Logger {
      file: Mutex::new(
        File::options()
          .create_new(true)
          .write(true)
          .open(format!("{}/{}-dopamine.txt", path, LocalChrono::now()).replace(':', "_"))
          .unwrap(),
      ),
    }
  }))?;

  log::set_max_level(LevelFilter::max());

  Ok(())
}

pub struct Logger {
  file: Mutex<File>,
}

impl Logger {
  pub const PATH: &str = "dopamine/logs";
}

impl Log for Logger {
  fn enabled(&self, _: &Metadata) -> bool {
    cfg!(debug_assertions)
  }

  fn log(&self, record: &Record) {
    if let Ok(fd) = &mut self.file.lock() {
      let _ = writeln!(
        fd,
        "[{}] in ({}):{} - {}",
        record.level(),
        record.file().unwrap_or("`unknown file`"),
        record.line().unwrap_or(u32::MAX),
        record.args()
      );
    }
  }

  fn flush(&self) {
    if let Ok(fd) = &mut self.file.lock() {
      let _ = fd.flush();
    }
  }
}
