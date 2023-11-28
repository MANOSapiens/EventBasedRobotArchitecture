// Import crates
extern crate ev3dev_lang_rust;


use log::{error, info};
use crate::consts::{LDRIVEENC, RDRIVEENC, LTOOLENC, RTOOLENC, CENTERBUTTON, DRIVEENC, LDRIVESPEED, RDRIVESPEED, LTOOLSPEED, RTOOLSPEED};
use std::time::{Instant};




// Local modules
use super::{
    COLOURSENS, DEBUG, GYRO, LDRIVECOR,
    RDRIVECOR,
};
use crate::Actuators::setMotorPow;
use crate::Events::{Event, FuncTypes};

use crate::ProcessLoop::SensorActuatorValues;
use crate::ReadSensors::getSensorValue;
use crate::PID::ComputePID;


fn MathFunc(inp: f32, func: &mut FuncTypes) -> f32 {
    match func {
        FuncTypes::ConstFunc { c } => *c,

        FuncTypes::LinearFunc {
            m,
            e,
            step_prev,
            lb,
            hb,
        } => {
            if *step_prev < 0.0 {
                *step_prev = inp;
            }

            let result: f32 = (inp - *step_prev) * (*m) + *e;
            if result > *hb {
                *hb
            } else if result < *lb {
                return *lb;
            } else {
                return (result * 50.0).round() / 50.0;
            }
        }

        FuncTypes::QuadFunc {
            a,
            b,
            c,
            step_prev,
            lb,
            hb,
        } => {
            if *step_prev == -1.0 {
                *step_prev = inp;
            }
            let x = inp - *step_prev;
            let result: f32 = (*a) * x.powf(2.0) + *b * x + *c;

            if result > *hb {
                *hb
            } else if result < *lb {
                return *lb;
            } else {
                return (result * 50.0).round() / 50.0;
            }
        }
    }
}

pub fn setReadSensor(sensor_id: i8, sensor_act_values: &mut SensorActuatorValues) 
{
    match sensor_id {
        LDRIVEENC => {
            sensor_act_values.lDriveMotorEncRead = true;
        }

        RDRIVEENC => {
            sensor_act_values.rDriveMotorEncRead = true;
        }

        LTOOLENC => {
            sensor_act_values.lToolMotorEncRead = true;
        }

        RTOOLENC => {
            sensor_act_values.rToolMotorEncRead = true;
        }

        LDRIVESPEED => {
            sensor_act_values.lDriveMotorSpeedRead = true;
        }

        RDRIVESPEED => {
            sensor_act_values.rDriveMotorSpeedRead = true;
        }

        LTOOLSPEED => {
            sensor_act_values.lToolMotorSpeedRead = true;
        }

        RTOOLSPEED => {
            sensor_act_values.rToolMotorSpeedRead = true;
        }

        GYRO => {
            sensor_act_values.gyroRead = true;
        }

        CENTERBUTTON => {
            sensor_act_values.centerButtonRead = true;
        }

        DRIVEENC => {
            sensor_act_values.lDriveMotorEncRead = true;
            sensor_act_values.rDriveMotorEncRead = true;
        }

        _ => {}
    }
}

pub fn RunEvents(
    event_list: &mut Vec<Event>,
    ActiveTable: &[bool],
    CondTable: &mut [bool],
    sensor_act_values: &mut SensorActuatorValues,
    sys_time: &Instant,
    running: &mut bool
) {
    sensor_act_values.currentTime = sys_time.elapsed().as_secs_f32();
    
    for _event in event_list {
        match _event {

            Event::SensorValue {
                event,
                sensor_target,
                sensor_prev,
                sensor_id,
                expr,
                sensvalcondid,
            } => {
                if ActiveTable[event.process_id] {
                    let sensor_value: f32 = getSensorValue(*sensor_id, sensor_act_values);
                    if *sensor_prev < -9998.0 {
                        *sensor_prev = sensor_value;
                    }

                    match expr {
                        '>' => {
                            CondTable[*sensvalcondid] =
                                sensor_value - *sensor_prev >= *sensor_target
                        }
                        '<' => {
                            CondTable[*sensvalcondid] =
                                sensor_value - *sensor_prev <= *sensor_target
                        }
                        _ => {
                            if DEBUG {
                                error!("Invalid character {} at Events::SensorValue", expr);
                            }
                        }
                    }

                    setReadSensor(*sensor_id, sensor_act_values);
                }
            }

            // Motor Control
            Event::MotorSpeedControl {
                event,
                motor_id,
                sensor_id,
                func,
            } => {
                if ActiveTable[event.process_id] {
                    setMotorPow(
                        MathFunc(getSensorValue(*sensor_id, sensor_act_values), func),
                        *motor_id,
                        sensor_act_values,
                    );
                }
            }

            // PID Control
            Event::PIDGyro {
                event,
                heading,
                pid,
                motor_correction,
                sensor_prev,
            } => {
                if ActiveTable[event.process_id] {
                    let sensor_value: f32 = getSensorValue(GYRO, sensor_act_values);
                    if *sensor_prev < -9998.0 {
                        *sensor_prev = sensor_value;
                    }

                    *motor_correction = ComputePID(sensor_value - *sensor_prev, heading, pid);
                    setMotorPow(-*motor_correction, LDRIVECOR, sensor_act_values);
                    setMotorPow(*motor_correction, RDRIVECOR, sensor_act_values);

                    sensor_act_values.gyroRead = true;
                }
            }

            Event::PIDLine {
                event,
                brightness_target,
                pid,
                motor_correction,
            } => {
                if ActiveTable[event.process_id] {
                    *motor_correction = ComputePID(
                        getSensorValue(COLOURSENS, sensor_act_values),
                        brightness_target,
                        pid,
                    );
                    setMotorPow(*motor_correction, LDRIVECOR, sensor_act_values);
                    setMotorPow(-*motor_correction, RDRIVECOR, sensor_act_values);
                }
            }

            Event::PIDHold { event, pid: _, motor_correction: _} => if ActiveTable[event.process_id] {},

            // Compute and Timer
            Event::ComputeMotorStall {
                event,
                min_mov_avg_speed,
                buffer,
                buffer_size,
                motor_id,
            } => {
                if ActiveTable[event.process_id] {
                    buffer.push_back(getSensorValue(*motor_id, sensor_act_values));
                    
                    if buffer.len() == *buffer_size {
                        let avg: f32 = buffer.iter().sum::<f32>() / *buffer_size as f32;
                        CondTable[event.term_conditions_id] = avg < *min_mov_avg_speed;

                        buffer.pop_front();
                    }
                    setReadSensor(*motor_id, sensor_act_values);
                }
            },

            Event::Timer {
                event,
                time,
                time_prev,
            } => {
                if ActiveTable[event.process_id] {
                    if *time_prev == -1.0 {
                        *time_prev = sys_time.elapsed().as_secs_f32();
                    } else {
                        let time_passed = sys_time.elapsed().as_secs_f32() - *time_prev;

                        if time_passed >= *time {
                            CondTable[event.term_conditions_id] = true;
                        }
                    }
                }
            }

            Event::HaltProcessLoop { 
                event 
            } => {
                if ActiveTable[event.process_id] {
                    *running = false;
                }
            }
            
            _ => {}
        }
    }
}
