use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

// Enum for log levels to ensure type safety
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

// Logger struct with configuration
#[derive(Debug)]
pub struct Logger {
    name: String,
    log_to_file: Option<String>, // Optional file path for logging
}

impl Logger {
    /// Creates a new Logger with a name and optional file output.
    pub fn new(name: String, log_to_file: Option<String>) -> Self {
        Logger { name, log_to_file }
    }

    /// Logs a message with the specified level.
    fn log(&self, level: LogLevel, message: &str) {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let level_str = match level {
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        };
        let log_message = format!("[{}] {} | {} - {}", now, self.name, level_str, message);

        // Output to console
        println!("{}", log_message);

        // Output to file if configured
        if let Some(file_path) = &self.log_to_file {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
            {
                let _ = writeln!(file, "{}", log_message);
            }
        }
    }

    /// Logs an info-level message.
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    /// Logs a warning-level message.
    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    /// Logs an error-level message.
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}