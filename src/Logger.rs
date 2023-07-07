use super::DEBUG;
use log::{info, error};
use log4rs;


pub fn init_logger(log_file: String) {
    log4rs::init_file(&log_file, Default::default()).unwrap();
    info!("===============================");
    info!("log4rs Logger Initialized!");
    info!("Will be using logger config file: {}", &log_file);

    // ...
}