// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port, Ev3Error};
use log::{error, info, warn};
use serde_json::error;
use std::time::{Duration, Instant};

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};

use std::{process, result};

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
            if DEBUG {
                error!(
                    "Motor ID {} unknown while assigning a power through setMotorPow()",
                    motor_id
                );
            }
        }
    }
}

pub fn ConstrainActuatorValues(value: &mut f32) {
    if *value > 1.0 {
        *value = 1.0;
    } else if *value < -1.0 {
        *value = -1.0;
    }
}

pub fn writeToActuators(motors_sensors: &MotorsSensors, sensor_act_values: &mut SensorActuatorValues) {
    let mut lDriveMotorPow: f32 = sensor_act_values.lDriveMotorPow + sensor_act_values.lDriveMotorCor;
    let mut rDriveMotorPow: f32 = sensor_act_values.rDriveMotorPow + sensor_act_values.rDriveMotorCor;
    let mut lToolMotorPow: f32 = sensor_act_values.lToolMotorPow + sensor_act_values.lToolMotorCor;
    let mut rToolMotorPow: f32 = sensor_act_values.rToolMotorPow + sensor_act_values.rToolMotorPow;

    //if lDriveMotorPow != sensor_act_values.lDriveMotorPowPrev {
        ConstrainActuatorValues(&mut lDriveMotorPow);
        
        let _ = motors_sensors.lDriveMotor.set_duty_cycle_sp((lDriveMotorPow * 100.0) as i32).expect("lDrive motor write failed");
        sensor_act_values.lDriveMotorPowPrev = lDriveMotorPow;
    //}

    //if rDriveMotorPow != sensor_act_values.rDriveMotorPowPrev {
        ConstrainActuatorValues(&mut rDriveMotorPow);
        
        motors_sensors.rDriveMotor.set_duty_cycle_sp((rDriveMotorPow * 100.0) as i32).expect("rDrive motor write failed");
        sensor_act_values.rDriveMotorPowPrev = rDriveMotorPow;
    //}

    //if lToolMotorPow != sensor_act_values.lToolMotorPowPrev {
        ConstrainActuatorValues(&mut lToolMotorPow);
        
        motors_sensors.lToolMotor.set_duty_cycle_sp((lToolMotorPow * 100.0) as i32).expect("lTool motor write failed");
        sensor_act_values.lToolMotorPowPrev = lToolMotorPow;
    //}

    //if rToolMotorPow != sensor_act_values.rToolMotorPowPrev {
        ConstrainActuatorValues(&mut rToolMotorPow);
        
        motors_sensors.rToolMotor.set_duty_cycle_sp((rToolMotorPow * 100.0) as i32).expect("rTool motor write failed");
        sensor_act_values.rToolMotorPowPrev = rToolMotorPow;
    //}
}