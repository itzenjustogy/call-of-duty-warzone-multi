use log::{LevelFilter, Log, Metadata, Record};

pub struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= LevelFilter::Info
    }

    fn log(&self, record: &Record) {
        println!("{}: {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_boxed_logger(Box::new(SimpleLogger)).unwrap();
    log::set_max_level(LevelFilter::Info);
}