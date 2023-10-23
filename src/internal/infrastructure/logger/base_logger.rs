pub trait Logger {
    fn info(&self, message: &str);
    fn error(&self, message: &str);
}

pub struct BaseLogger {
    pub color: bool,
    pub verbose: bool,
}

impl BaseLogger {
    pub fn new() -> Self {
        Self
    }
}

impl Logger for BaseLogger {
    fn info(&self, message: &str) {
        eprintln!("{message}");
    }

    fn error(&self, message: &str) {
        eprintln!("{}: {}", "error".red().bold(), message);
    }
}
