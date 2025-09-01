use super::{log4rs::Log4rsEngine, slog::SlogEngine, tracing::TracingEngine};



enum LoggerType {
    LOG4RS(Log4rsEngine),
    SLOG(SlogEngine),
    TRACING(TracingEngine),
}
pub trait LoggerProvider {
    fn init(&self);
}



fn init(logger: LoggerType) {
    match logger {
        LoggerType::LOG4RS(log4rs_engine) => log4rs_engine.init(),
        LoggerType::SLOG(slog_engine) => slog_engine.init(),
        LoggerType::TRACING(tracing_engine) => tracing_engine.init(),
    }
}

pub fn initialize(){
    let _slog = LoggerType::SLOG(SlogEngine);
    let _log4rs = LoggerType::LOG4RS(Log4rsEngine);
    let _tracing = LoggerType::TRACING(TracingEngine);

    init(_log4rs);
}