use super::DEBUG;
use log::{info, error};
use log4rs;


pub fn init_logger(log_file: String) {
    log4rs::init_file(&log_file, Default::default()).unwrap();

}