use super::logger::Logger;

pub struct TestLogging {
    logger: Logger,
}

impl TestLogging {
    pub fn new() -> Self {
        TestLogging {
            logger: Logger::new(file!()),
        }
    }

    pub fn test_logging(&self) {
        self.logger.info("It's just a test.");
    }
}