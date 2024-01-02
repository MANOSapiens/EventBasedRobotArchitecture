// Import crates
extern crate ev3dev_lang_rust;


use ev3dev_lang_rust::motors;
use log::{info, error};
use Logger::logCSV;
use std::time::{Instant};
use std::io::Write;




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
    //let _ = motors_sensors.gyroSens.set_mode_gyro_ang();
}

pub fn getSensorValue(sensor_id: i8, sensor_act_values: &mut SensorActuatorValues) -> f32{
    match sensor_id {
        DRIVEENC => (sensor_act_values.lDriveMotorEnc + sensor_act_values.rDriveMotorEnc)/2.0,
        LDRIVEENC => sensor_act_values.lDriveMotorEnc,
        RDRIVEENC => sensor_act_values.rDriveMotorEnc,
        LTOOLENC => sensor_act_values.lToolMotorEnc,
        RTOOLENC => sensor_act_values.rToolMotorEnc,
        DRIVESPEED => (sensor_act_values.lDriveMotorSpeed + sensor_act_values.rDriveMotorSpeed)/2.0,
        LDRIVESPEED => sensor_act_values.lDriveMotorSpeed,
        RDRIVESPEED => sensor_act_values.rDriveMotorSpeed,
        LTOOLSPEED => sensor_act_values.lToolMotorSpeed,
        RTOOLSPEED => sensor_act_values.rToolMotorSpeed,
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
        RIGHTBUTTON => sensor_act_values.rightButton,
        TIME => sensor_act_values.currentTime,
        PREVTIME => sensor_act_values.timePrev,
        
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
    sensor_act_values.gyroRate = 0.0;

    if sensor_act_values.rDriveMotorEncRead && sensor_act_values.lDriveMotorEncRead{
        sensor_act_values.driveMotorEncPrev = getSensorValue(DRIVEENC, sensor_act_values)
    }

    if sensor_act_values.lDriveMotorEncRead {
        sensor_act_values.lDriveMotorEnc = motors_sensors.lDriveMotor.get_position().expect("lDriveEnc failed") as f32;
    }

    if sensor_act_values.rDriveMotorEncRead {
        sensor_act_values.rDriveMotorEnc = motors_sensors.rDriveMotor.get_position().expect("rDriveEnc failed") as f32;
    }

    if sensor_act_values.lToolMotorEncRead {
        sensor_act_values.lToolMotorEnc = motors_sensors.lToolMotor.get_position().unwrap_or(0) as f32;
    }

    if sensor_act_values.rToolMotorEncRead {
        sensor_act_values.rToolMotorEnc = motors_sensors.rToolMotor.get_position().unwrap_or(0) as f32; 
    }

    if sensor_act_values.lDriveMotorSpeedRead {
        sensor_act_values.lDriveMotorSpeed = motors_sensors.lDriveMotor.get_speed().expect("lDriveSpeed failed") as f32;
    }

    if sensor_act_values.rDriveMotorSpeedRead {
        sensor_act_values.rDriveMotorSpeed = motors_sensors.rDriveMotor.get_speed().expect("rDriveSpeed failed") as f32;
    }

    if sensor_act_values.lToolMotorSpeedRead {
        sensor_act_values.lToolMotorSpeed = motors_sensors.lToolMotor.get_speed().unwrap_or(0) as f32;
    }

    if sensor_act_values.rToolMotorSpeedRead {
        sensor_act_values.rToolMotorSpeed = motors_sensors.rToolMotor.get_speed().unwrap_or(0) as f32; 
    }
    
    if sensor_act_values.gyroRead {
        sensor_act_values.gyroAngValue = motors_sensors.gyroSens.get_angle().expect("gyro ang failed") as f32 - sensor_act_values.gyroAngValuePrev;
        //sensor_act_values.gyroRate = motors_sensors.gyroSens.get_rotational_speed().expect("gyro rate failed") as f32;
        
    }
    
    if sensor_act_values.rightButtonRead {
        motors_sensors.button.process();
        sensor_act_values.rightButton = (motors_sensors.button.is_right() as i32) as f32;
    }

    if DEBUG {
        logCSV(wtr, sensor_act_values).expect("cant write to CSV file!");
    }

    sensor_act_values.timePrev = sensor_act_values.currentTime;
    sensor_act_values.currentTime = sys_time.elapsed().as_secs_f32();
    
    // ===== Reset Actuator Variables =====
    sensor_act_values.lDriveMotorPow = 0.0;
    sensor_act_values.rDriveMotorPow = 0.0;
    sensor_act_values.lToolMotorPow = 0.0;
    sensor_act_values.rToolMotorPow = 0.0;

    sensor_act_values.lDriveMotorCor = 0.0;
    sensor_act_values.rDriveMotorCor = 0.0;
    sensor_act_values.lToolMotorCor = 0.0;
    sensor_act_values.rToolMotorCor = 0.0;

    sensor_act_values.lDriveMotorEncRead = !SPARSE_SENSOR_READING;
    sensor_act_values.rDriveMotorEncRead = !SPARSE_SENSOR_READING;
    sensor_act_values.lToolMotorEncRead = !SPARSE_SENSOR_READING;
    sensor_act_values.rToolMotorEncRead = !SPARSE_SENSOR_READING;

    sensor_act_values.lDriveMotorSpeedRead = true;
    sensor_act_values.rDriveMotorSpeedRead = true;
    sensor_act_values.lToolMotorSpeedRead = !SPARSE_SENSOR_READING;
    sensor_act_values.rToolMotorSpeedRead = !SPARSE_SENSOR_READING;
    
    sensor_act_values.gyroRead = !SPARSE_SENSOR_READING;
    sensor_act_values.rightButtonRead = !SPARSE_SENSOR_READING;
}