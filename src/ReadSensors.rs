// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port};
use log::{error, info, warn};
use std::time::{Duration, Instant};

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};

// Local modules
use super::*;
use crate::Events::{Condition, Event};
use crate::Ports::{MotorsSensors, PortDefinition};
use crate::ProcessLoop::{SensorActuatorValues};

//  =============== RESET ==================
pub fn resetAll(motors_sensors: &MotorsSensors) {
    motors_sensors.lDriveMotor.set_position(0);
    motors_sensors.rDriveMotor.set_position(0);
    motors_sensors.lToolMotor.set_position(0);
    motors_sensors.rToolMotor.set_position(0);
}

pub fn getSensorValue(sensor_id: i8, sensor_act_values: &mut SensorActuatorValues) -> f32{
    match sensor_id {
        DRIVEENC => return (sensor_act_values.lDriveMotorEnc + sensor_act_values.rDriveMotorEnc)/2.0,
        LDRIVEENC => return sensor_act_values.lDriveMotorEnc,
        RDRIVEENC => return sensor_act_values.rDriveMotorEnc,
        LTOOLENC => return sensor_act_values.lToolMotorEnc,
        RTOOLENC => return sensor_act_values.rToolMotorEnc,
        COLOURSENS => return sensor_act_values.colourSensValue,
        GYRO => return sensor_act_values.gyroAngValue,
        LDRIVEPOW => return sensor_act_values.lDriveMotorPow,
        RDRIVEPOW => return sensor_act_values.rDriveMotorPow,
        LTOOLPOW => return sensor_act_values.lToolMotorPow,
        RTOOLPOW => return sensor_act_values.rToolMotorPow,
        LDRIVECOR => return sensor_act_values.lDriveMotorCor,
        RDRIVECOR => return sensor_act_values.rDriveMotorCor,
        LTOOLCOR => return sensor_act_values.lToolMotorCor,
        RTOOLCOR => return sensor_act_values.rToolMotorCor,
        LDRIVESPEED => return sensor_act_values.lDriveMotorSpeed,
        RDRIVESPEED => return sensor_act_values.rDriveMotorSpeed,
        DRIVESPEED => return (sensor_act_values.rDriveMotorSpeed + sensor_act_values.lDriveMotorSpeed)/2.0,
        LTOOLSPEED => return sensor_act_values.lToolMotorSpeed,
        RTOOLSPEED => return sensor_act_values.rToolMotorSpeed,
        CENTERBUTTON => return sensor_act_values.centerButton,
        TIME => return sensor_act_values.currentTime,
        _ => {
            if DEBUG {
                error!("Sensor ID {} unknown while searching a value through getSensorValue()", sensor_id);
            }
            return 0.0
        }
    }
}

/* async fn lDrive(motors_sensors: &MotorsSensors) -> f32 {
    motors_sensors.lDriveMotor.get_position().expect("lDriveEnc failed") as f32
}

async fn rDrive(motors_sensors: &MotorsSensors) -> f32 {
    motors_sensors.rDriveMotor.get_position().expect("rDriveEnc failed") as f32
}

async fn lTool(motors_sensors: &MotorsSensors) -> f32 {
    motors_sensors.lToolMotor.get_position().expect("lToolEnc failed") as f32
}

async fn rTool(motors_sensors: &MotorsSensors) -> f32 {
    motors_sensors.rToolMotor.get_position().expect("rToolEnc failed") as f32
}

async fn gyro(motors_sensors: &MotorsSensors) -> f32 {
    motors_sensors.gyroSens.get_angle().expect("gyro failed") as f32
}

async fn colour(motors_sensors: &MotorsSensors) -> f32 {
    motors_sensors.colourSens.get_color().expect("colour sensor failed") as f32
}

async fn readSensorValues(motors_sensors: &MotorsSensors) -> (f32, f32, f32, f32, f32, f32) {
    join!(lDrive(motors_sensors), rDrive(motors_sensors), lTool(motors_sensors), rTool(motors_sensors), gyro(motors_sensors), colour(motors_sensors))
} */

pub fn ReadSensors(
    motors_sensors: &MotorsSensors,
    sensor_act_values: &mut SensorActuatorValues,
    sys_time: &Instant,
    read_sensor_last_time: &mut f32,
) {
    sensor_act_values.lDriveMotorEnc = motors_sensors.lDriveMotor.get_position().expect("lDriveEnc failed") as f32;
    sensor_act_values.rDriveMotorEnc = motors_sensors.rDriveMotor.get_position().expect("rDriveEnc failed") as f32;
    sensor_act_values.lToolMotorEnc = motors_sensors.lToolMotor.get_position().expect("lToolEnc failed") as f32;
    sensor_act_values.rToolMotorEnc = motors_sensors.rToolMotor.get_position().expect("rToolEnc failed") as f32; 
    sensor_act_values.gyroAngValue = motors_sensors.gyroSens.get_angle().expect("gyro failed") as f32;
    sensor_act_values.colourSensValue = motors_sensors.colourSens.get_color().expect("colour sensor failed") as f32;
    sensor_act_values.centerButton = (motors_sensors.button.is_enter() as i32) as f32;

    /* let sensorResult: (f32, f32, f32, f32, f32, f32) = block_on(readSensorValues(motors_sensors));

    sensor_act_values.lDriveMotorEnc = sensorResult.0;
    sensor_act_values.rDriveMotorEnc = sensorResult.1;
    sensor_act_values.lToolMotorEnc = sensorResult.2;
    sensor_act_values.rToolMotorEnc = sensorResult.3;

    sensor_act_values.gyroAngValue = sensorResult.4;
    sensor_act_values.colourSensValue = sensorResult.5; */

    let time: f32 = sys_time.elapsed().as_secs_f32();
    let time_elapsed: f32 = time - *read_sensor_last_time;

    sensor_act_values.lDriveMotorSpeed = (sensor_act_values.lDriveMotorEnc-sensor_act_values.lDriveMotorEncPrev) / time_elapsed;
    sensor_act_values.lDriveMotorEncPrev = sensor_act_values.lDriveMotorEnc;

    sensor_act_values.rDriveMotorSpeed = (sensor_act_values.rDriveMotorEnc-sensor_act_values.rDriveMotorEncPrev) / time_elapsed;
    sensor_act_values.rDriveMotorEncPrev = sensor_act_values.rDriveMotorEnc;
    
    sensor_act_values.lToolMotorSpeed = (sensor_act_values.lToolMotorEnc-sensor_act_values.lToolMotorEncPrev) / time_elapsed;
    sensor_act_values.lToolMotorEncPrev = sensor_act_values.lToolMotorEnc;

    sensor_act_values.rToolMotorSpeed = (sensor_act_values.rToolMotorEnc-sensor_act_values.rToolMotorEncPrev) / time_elapsed;
    sensor_act_values.rToolMotorEncPrev = sensor_act_values.rToolMotorEnc;
    *read_sensor_last_time = time;
}