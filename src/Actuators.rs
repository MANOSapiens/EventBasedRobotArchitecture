// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port};
use log::{error, info, warn};
use std::time::{Duration, Instant};

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};

use std::process;

// Local modules
use super::{
    COLOURSENS, DEBUG, GYRO, LDRIVECOR, LDRIVEENC, LDRIVEPOW, LTOOLCOR, LTOOLENC, LTOOLPOW,
    RDRIVECOR, RDRIVEENC, RDRIVEPOW, RTOOLCOR, RTOOLENC, RTOOLPOW,
};
use crate::Events::{Condition, Event};
use crate::Ports::{MotorsSensors, PortDefinition};
use crate::ProcessLoop::SensorActuatorValues;

// Motor control helper
pub fn setMotorPow(motor_pow: f32, motor_id: i8, sensor_act_values: &mut SensorActuatorValues) {
    match motor_id {
        LDRIVEPOW => sensor_act_values.lDriveMotorPow = motor_pow,
        RDRIVEPOW => sensor_act_values.rDriveMotorPow = motor_pow,
        LTOOLPOW => sensor_act_values.lToolMotorPow = motor_pow,
        RTOOLPOW => sensor_act_values.rToolMotorPow = motor_pow,

        LDRIVECOR => sensor_act_values.lDriveMotorCor = motor_pow,
        RDRIVECOR => sensor_act_values.rDriveMotorCor = motor_pow,
        LTOOLCOR => sensor_act_values.lToolMotorCor = motor_pow,
        RTOOLCOR => sensor_act_values.rToolMotorCor = motor_pow,
        _ => {
            error!(
                "Motor ID {} unknown while assigning a power through setMotorPow()",
                motor_id
            );
            process::exit(0);
        }
    }
}

pub fn ConstrainActuatorValues(value: &mut f32) -> bool {
    if *value > 1.0 {
        *value = 1.0;
        false
    } else if *value < -1.0 {
        *value = -1.0;
        false
    } else {
        true
    }
}

pub fn writeToActuators(motors_sensors: &mut MotorsSensors, sensor_act_values: &mut SensorActuatorValues) {
    let mut lDriveMotorPow: f32 = sensor_act_values.lDriveMotorPow + sensor_act_values.lDriveMotorCor;
    let mut rDriveMotorPow: f32 = sensor_act_values.rDriveMotorPow + sensor_act_values.rDriveMotorCor;
    let mut lToolMotorPow: f32 = sensor_act_values.lToolMotorPow + sensor_act_values.lToolMotorCor;
    let mut rToolMotorPow: f32 = sensor_act_values.rToolMotorPow + sensor_act_values.rToolMotorPow;

    if lDriveMotorPow != sensor_act_values.lDriveMotorPowPrev {
        if DEBUG && !ConstrainActuatorValues(&mut lDriveMotorPow) {
            warn!("The motor power of the left drive motor is out of range!")
        }
        let _ = motors_sensors.lDriveMotor.set_duty_cycle_sp((lDriveMotorPow * 100.0) as i32);
        sensor_act_values.lDriveMotorPowPrev = lDriveMotorPow;
    }

    if rDriveMotorPow != sensor_act_values.rDriveMotorPowPrev {
        if DEBUG && !ConstrainActuatorValues(&mut rDriveMotorPow) {
            warn!("The motor power of the right drive motor is out of range!")
        }
        let _ = motors_sensors.rDriveMotor.set_duty_cycle_sp((rDriveMotorPow * 100.0) as i32);
        sensor_act_values.rDriveMotorPowPrev = rDriveMotorPow;
    }

    if lToolMotorPow != sensor_act_values.lToolMotorPowPrev {
        if DEBUG && !ConstrainActuatorValues(&mut lToolMotorPow) {
            warn!("The motor power of the left tool motor is out of range!")
        }
        let _ = motors_sensors.lToolMotor.set_duty_cycle_sp((lToolMotorPow * 100.0) as i32);
        sensor_act_values.lToolMotorPowPrev = lToolMotorPow;
    }

    if rToolMotorPow != sensor_act_values.rToolMotorPowPrev {
        if DEBUG && !ConstrainActuatorValues(&mut rToolMotorPow) {
            warn!("The motor power of the right tool motor is out of range!")
        }
        let _ = motors_sensors.rToolMotor.set_duty_cycle_sp((rToolMotorPow * 100.0) as i32);
        sensor_act_values.rToolMotorPowPrev = rToolMotorPow;
    }

    // ===== Reset Actuator Variables =====
    sensor_act_values.lDriveMotorPow = 0.0;
    sensor_act_values.rDriveMotorPow = 0.0;
    sensor_act_values.lToolMotorPow = 0.0;
    sensor_act_values.rToolMotorPow = 0.0;

    sensor_act_values.lDriveMotorCor = 0.0;
    sensor_act_values.rDriveMotorCor = 0.0;
    sensor_act_values.lToolMotorCor = 0.0;
    sensor_act_values.rToolMotorCor = 0.0;
}