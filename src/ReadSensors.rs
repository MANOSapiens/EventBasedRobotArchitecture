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

pub fn getSensorValue(sensor_id: &i8, sensor_act_values: &mut SensorActuatorValues) -> i32{
    match sensor_id {
        0 => return sensor_act_values.lDriveMotorEnc,
        1 => return sensor_act_values.rDriveMotorEnc,
        2 => return sensor_act_values.lToolMotorEnc,
        3 => return sensor_act_values.rToolMotorEnc,
        4 => return sensor_act_values.colourSensValue,
        5 => return sensor_act_values.gyroAngValue,
        6 => return sensor_act_values.lDriveMotorPow,
        7 => return sensor_act_values.rDriveMotorPow,
        8 => return sensor_act_values.lToolMotorPow,
        9 => return sensor_act_values.rToolMotorPow,
        _ => {
            error!("Sensor ID {} unknown while searching a value through getSensorValue()", sensor_id);
            return 0
        }
    }
}

pub fn ReadSensors(
    motors_sensors: &mut MotorsSensors,
    sensor_act_values: &mut SensorActuatorValues,
    debug: &bool,
) {
}