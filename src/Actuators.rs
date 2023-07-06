// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port};
use log::{error, info, warn};
use std::time::{Duration, Instant};

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};

// Local modules
use crate::Events::{Condition, Event};
use crate::Ports::{MotorsSensors, PortDefinition};
use crate::ProcessLoop::{SensorActuatorValues};

// Motor control helper
fn setMotorPow(motor_pow: i32, motor_id: i8, sensor_act_values: &mut SensorActuatorValues) {
    match motor_pow {
        0 => {sensor_act_values.lDriveMotorPow = motor_pow},
        1 => {sensor_act_values.rDriveMotorPow = motor_pow},
        2 => {sensor_act_values.lToolMotorPow = motor_pow},
        3 => {sensor_act_values.rToolMotorPow = motor_pow},
        _ => {
            error!("Motor ID {} unknown while assigning a power through setMotorPow()", motor_pow);
        }
    }
}

