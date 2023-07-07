// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port};
use log::{error, info, warn};
use std::time::{Duration, Instant};

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};

use std::process;

// Local modules
use super::DEBUG;
use crate::Events::{Condition, Event};
use crate::Ports::{MotorsSensors, PortDefinition};
use crate::ProcessLoop::{SensorActuatorValues};

// Motor control helper
pub fn setMotorPow(motor_pow: f32, motor_id: i8, sensor_act_values: &mut SensorActuatorValues) {
    match motor_id {
        6 => {sensor_act_values.lDriveMotorPow = motor_pow},
        7 => {sensor_act_values.rDriveMotorPow = motor_pow},
        8 => {sensor_act_values.lToolMotorPow = motor_pow},
        9 => {sensor_act_values.rToolMotorPow = motor_pow},

        10 => {sensor_act_values.lDriveMotorCor = motor_pow},
        11 => {sensor_act_values.rDriveMotorCor = motor_pow},
        12 => {sensor_act_values.lToolMotorCor = motor_pow},
        13 => {sensor_act_values.rToolMotorCor = motor_pow},
        _ => {
            error!("Motor ID {} unknown while assigning a power through setMotorPow()", motor_id);
            process::exit(0);
        }
    }
}

