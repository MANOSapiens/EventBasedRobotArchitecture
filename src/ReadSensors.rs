// Import crates
extern crate ev3dev_lang_rust;


use ev3dev_lang_rust::motors;
use log::{error};
use Logger::logCSV;
use std::time::{Instant};
use std::io::Write;
use simple_moving_average::{SingleSumSMA, SumTreeSMA};




// Local modules
use super::*;

use crate::Ports::{MotorsSensors};
use crate::ProcessLoop::{SensorActuatorValues};

//  =============== RESET ==================
pub fn resetAll(motors_sensors: &MotorsSensors) {
    let _ = motors_sensors.lDriveMotor.set_position(0);
    let _ = motors_sensors.rDriveMotor.set_position(0);
    let _ = motors_sensors.lToolMotor.set_position(0);
    let _ = motors_sensors.rToolMotor.set_position(0);
    let _ = motors_sensors.gyroSens.set_mode_gyro_ang();
}

pub fn getSensorValue(sensor_id: i8, sensor_act_values: &mut SensorActuatorValues) -> f32{
    match sensor_id {
        DRIVEENC => (sensor_act_values.lDriveMotorEnc + sensor_act_values.rDriveMotorEnc)/2.0,
        LDRIVEENC => sensor_act_values.lDriveMotorEnc,
        RDRIVEENC => sensor_act_values.rDriveMotorEnc,
        LTOOLENC => sensor_act_values.lToolMotorEnc,
        RTOOLENC => sensor_act_values.rToolMotorEnc,
        COLOURSENS => sensor_act_values.colourSensValue,
        GYRO => sensor_act_values.gyroAngValue,
        LDRIVEPOW => sensor_act_values.lDriveMotorPow,
        RDRIVEPOW => sensor_act_values.rDriveMotorPow,
        LTOOLPOW => sensor_act_values.lToolMotorPow,
        RTOOLPOW => sensor_act_values.rToolMotorPow,
        LDRIVECOR => sensor_act_values.lDriveMotorCor,
        RDRIVECOR => sensor_act_values.rDriveMotorCor,
        LTOOLCOR => sensor_act_values.lToolMotorCor,
        RTOOLCOR => sensor_act_values.rToolMotorCor,
        CENTERBUTTON => sensor_act_values.centerButton,
        TIME => sensor_act_values.currentTime,
        _ => {
            if DEBUG {
                error!("Sensor ID {} unknown while searching a value through getSensorValue()", sensor_id);
            }
            0.0
        }
    }
}

pub fn ReadSensors<W: Write>(
    motors_sensors: &MotorsSensors,
    sensor_act_values: &mut SensorActuatorValues,
    sys_time: &Instant,
    wtr: &mut csv::Writer<W>

) {
    sensor_act_values.lDriveMotorEnc = motors_sensors.lDriveMotor.get_position().expect("lDriveEnc failed") as f32;
    sensor_act_values.rDriveMotorEnc = motors_sensors.rDriveMotor.get_position().expect("rDriveEnc failed") as f32;
    //sensor_act_values.lToolMotorEnc = motors_sensors.lToolMotor.get_position().expect("lToolEnc failed") as f32;
    //sensor_act_values.rToolMotorEnc = motors_sensors.rToolMotor.get_position().expect("rToolEnc failed") as f32; 
    //sensor_act_values.gyroAngValue = motors_sensors.gyroSens.get_angle().expect("gyro failed") as f32 - sensor_act_values.gyroAngValuePrev;
    
    //sensor_act_values.colourSensValue = motors_sensors.colourSens.get_color().expect("colour sensor failed") as f32;
    //sensor_act_values.centerButton = (motors_sensors.button.is_enter() as i32) as f32;

    sensor_act_values.currentTime = sys_time.elapsed().as_secs_f32();

    *read_sensor_last_time = time;

    if DEBUG {
        logCSV(wtr, sensor_act_values).expect("cant write to CSV file!");
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

pub fn CalculateSpeed<T>(
    sensor_act_values: &mut SensorActuatorValues, 
    sys_time: &Instant,
    lDriveSMA: SumTreeSMA::<T, f32, 10>,
    rDriveSMA: SumTreeSMA::<T, f32, 10>
) {
    //sensor_act_values.lDriveMotorSpeed = motors_sensors.lDriveMotor.get_speed().expect("lDriveSpeed failed") as f32;

    //sensor_act_values.rDriveMotorSpeed = motors_sensors.rDriveMotor.get_speed().expect("rDriveSpeed failed") as f32;
    
    //sensor_act_values.lToolMotorSpeed = motors_sensors.lToolMotor.get_speed().expect("lToolSpeed failed") as f32;

    //sensor_act_values.rToolMotorSpeed = motors_sensors.rToolMotor.get_speed().expect("rToolSpeed failed") as f32;
}