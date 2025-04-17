use chrono::Local;
use colored::Colorize;

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
}

impl Logger {
    /// Creates a new Logger with a name.
    pub fn new(name: &str) -> Self {
        Logger { name: name.to_string() }
    }

    /// Logs a message with the specified level to the console.
    fn log(&self, level: LogLevel, message: &str) {
        let now = Local::now().format("%H:%M:%S").to_string();
        let level_str = match level {
            LogLevel::Info => "INFO".green(),
            LogLevel::Warn => "WARN".yellow(),
            LogLevel::Error => "ERROR".red(),
        };
        // Pad name to 5 characters, level_str to 5 characters; gray brackets
        let log_message = format!(
            "{}{}{} {}{:5}{} {}{:5}{} {}",
            "[".bright_black(),
            now,
            "]".bright_black(),
            "[".bright_black(),
            self.name,
            "]".bright_black(),
            "[".bright_black(),
            level_str,
            "]".bright_black(),
            message
        );

        // Output to console
        println!("{}", log_message);
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

    pub fn basic(&self, message: &str){
        println!("{}", message);
    }

    // Clear the console
    pub fn clear(&self){
        print!("\x1B[2J\x1B[1;1H");
    }
}