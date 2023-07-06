// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port};
use log::{error, info, warn};
use std::time::{Duration, Instant};

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};


pub fn Check(sys_time: &Instant, last_time: &mut f64, loop_counter: &mut i32, debug: &bool) {
    let now: f64 = sys_time.elapsed().as_secs_f64();
    let elapsed: f64 = now - *last_time;
    
}