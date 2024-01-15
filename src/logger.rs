use log::{LevelFilter, SetLoggerError};
use env_logger::Env;



pub fn initialize_logger() -> Result<(), SetLoggerError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).filter_level(LevelFilter::Info).init();
    Ok(())
}