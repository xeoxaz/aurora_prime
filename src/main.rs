mod logger;
use logger::Logger;

mod test;


fn main() {
    let logger = Logger::new(file!());

    logger.clear();

    logger.basic("");
    logger.basic("Aurora Prime");
    logger.basic("");

    logger.info("Application started.");
    logger.warn("Low memory warning.");
    logger.error("Database connection failed.");

    test::TestLogging::new().test_logging();
}