use super::main::LoggerProvider;


pub struct Log4rsEngine;
impl LoggerProvider for Log4rsEngine {
    fn init(&self) {
        log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    }
}