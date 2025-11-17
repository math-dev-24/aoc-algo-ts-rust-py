use std::fmt::Display;
use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;

pub struct Logger {
    level: LogLevel,
    logs: Vec<LogDetail>
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Self { level, logs: Vec::new() }
    }
    pub fn is_debug(&self) -> bool {
        self.level == LogLevel::Debug
    }
    pub fn post(&mut self, log: LogDetail) {
        match self.level {
            LogLevel::Info => println!("{}", log.msg),
            LogLevel::Debug => println!("[{}] ### {}", self.level, log.msg),
        }
        self.logs.push(log);
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create("logs.txt")?;
        for log in &self.logs {
            writeln!(file, "[{}] ### {}", self.level, log.msg)?;
        }
        Ok(())
    }
}



#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LogLevel {
    Info,
    Debug,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct  LogDetail {
    pub msg: String,
    pub timestamp: Instant
}

