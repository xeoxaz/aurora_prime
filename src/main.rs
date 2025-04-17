mod logger;
use logger::Logger;

fn main() {
    // Console-only logger
    let logger = Logger::new("MyApp".to_string(), None);
    logger.info("Application started.");
    logger.warn("Low memory warning.");
    logger.error("Database connection failed.");

    // Logger with file output
    let file_logger = Logger::new("MyApp".to_string(), Some("app.log".to_string()));
    file_logger.info("Logging to file.");
}