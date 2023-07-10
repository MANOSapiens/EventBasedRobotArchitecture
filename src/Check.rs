// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port};
use log::{error, info, warn};
use std::time::{Duration, Instant};

use crate::ProcessLoop::SensorActuatorValues;

use super::DEBUG;
use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};

pub struct RoundSummary {
    pub wall_time: u64,
    pub max_loop_time: f64,
    pub mean_loop_time: f64,
    pub total_travelled_distance: i32,
}

pub fn Check(
    sys_time: &Instant,
    last_time: &mut f64,
    round_summary: &mut RoundSummary,
    CondTable: &mut Vec<bool>,
    sensor_act_values: &SensorActuatorValues,
) {
    let now: f64 = sys_time.elapsed().as_secs_f64();
    let elapsed: f64 = now - *last_time;
    *last_time = now;

    // Take down maximum loop time
    if round_summary.max_loop_time < elapsed {
        round_summary.max_loop_time = elapsed;
    }

    round_summary.mean_loop_time = (elapsed + round_summary.mean_loop_time) / 2.0;
    
    // Check sensor values
}
