use std::fmt::Display;
use std::io::Write;
use std::{fs::OpenOptions, sync::Mutex};

use colored::Colorize;

pub mod macros;

pub enum Channel {
    Stdout,
    Stderr,
    File(String),
}

pub enum Level {
    Info,
    Warning,
    Error,
    Trace,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Info => write!(f, "{}", "INFO".on_blue().white().bold()),
            Level::Warning => write!(f, "{}", "WARNING".on_yellow().white().bold()),
            Level::Error => write!(f, "{}", "ERROR".on_red().white().bold()),
            Level::Trace => write!(f, "{}", "TRACE".on_purple().white().bold()),
        }
    }
}

pub struct Logger {
    channel: Channel,
}

lazy_static::lazy_static! {
    static ref GLOBAL_LOGGER: Mutex<Logger> = Mutex::new(Logger::new(Channel::Stdout));
}

impl Logger {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    pub fn set_channel(channel: Channel) {
        let mut logger = GLOBAL_LOGGER.lock().unwrap();
        logger.channel = channel;
    }

    fn dispatch(log: &str) {
        let logger = GLOBAL_LOGGER.lock().unwrap();
        match &logger.channel {
            Channel::Stdout => println!("{log}"),
            Channel::Stderr => eprintln!("{log}"),
            Channel::File(path) => {
                if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
                    writeln!(file, "{log}").ok();
                }
            }
        }
    }

    pub fn log(level: Level, msg: &str) {
        let formated = format!("{:^8} {:20}", level, msg);
        Self::dispatch(&formated);
    }
}
