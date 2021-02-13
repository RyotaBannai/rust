use derive_more::From;
use once_cell::sync::{Lazy, OnceCell};
use std::sync::{Arc, Mutex};
use std::{env, io, num};

static LARGE_TEST: Lazy<String> = Lazy::new(|| load_large_text());

fn load_large_text() -> String {
  "So large text".to_string()
}

#[derive(Debug)]
pub struct Logger {
  logLevel: Arc<Mutex<LogLevel>>,
}

#[derive(Copy, Clone, Debug)]
enum LogLevel {
  DEBUG,
  INFO,
  WARNING,
  ERROR,
}

// https://doc.rust-lang.org/std/convert/trait.From.html
#[derive(Debug, From)]
pub enum CliError {
  IoError(io::Error),
  ParseError(num::ParseIntError),
}

// with derive_from you don't need to impl below Errors.
// impl From<io::Error> for CliError {
//   fn from(error: io::Error) -> Self {
//     CliError::IoError(error)
//   }
// }

// impl From<num::ParseIntError> for CliError {
//   fn from(error: num::ParseIntError) -> Self {
//     CliError::ParseError(error)
//   }
// }

static INSTANCE: OnceCell<Logger> = OnceCell::new();

impl Logger {
  pub fn global() -> &'static Logger {
    INSTANCE.get().expect("logger is not initialized yet.")
  }

  pub fn from_cli(args: env::Args) -> Result<(), CliError> {
    let logger = Logger {
      logLevel: Arc::new(Mutex::new(LogLevel::DEBUG)),
    };
    let mut next_is_option_flag = false;
    for argument in args {
      if argument == "-l" {
        next_is_option_flag = true;
        // continue;
      }
      if next_is_option_flag == true {
        let log_level = argument.parse::<i32>()?;
        let mut temp_logger = logger.logLevel.lock().unwrap();
        *temp_logger = match log_level {
          0 => LogLevel::DEBUG,
          1 => LogLevel::INFO,
          2 => LogLevel::WARNING,
          3 => LogLevel::ERROR,
          _ => panic!("\nSet from:\n 0: INFO\n 1: DEBUG\n 2: WARNING\n 3: ERROR\n"),
        };
        break;
      }
    }
    INSTANCE.set(logger);
    Ok(())
  }
}

pub fn test() {
  // println!("{}", *LARGE_TEST);

  Logger::from_cli(env::args())
    .map_err(|err| println!("{:?}", err))
    .ok(); // Result to Option
           // .unwrap_or_else(count)
  dbg!(Logger::global());
}
