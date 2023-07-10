// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port};
use log::{error, info, warn};
use std::time::{Duration, Instant};

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};

// Local modules
use super::{
    COLOURSENS, DEBUG, GYRO, LDRIVECOR, LDRIVEENC, LDRIVEPOW, LTOOLCOR, LTOOLENC, LTOOLPOW,
    RDRIVECOR, RDRIVEENC, RDRIVEPOW, RTOOLCOR, RTOOLENC, RTOOLPOW,
};
use crate::Actuators::setMotorPow;
use crate::Events::{Condition, Event, FuncTypes};
use crate::Ports::{MotorsSensors, PortDefinition};
use crate::ProcessLoop::SensorActuatorValues;
use crate::ReadSensors::getSensorValue;
use crate::PID::ComputePID;

fn MathFuncBasedOnTime(time: f32, func: &mut FuncTypes) -> f32 {
    match func {
        FuncTypes::ConstFunc { c } => return *c,

        FuncTypes::LinearFunc {
            m,
            e,
            step_prev,
            lb,
            hb,
        } => {
            if *step_prev < 0.0 {
                *step_prev = time;
            }

            let result: f32 = (time - *step_prev) * (*m) + *e;
            if result > *hb {
                return *hb;
            } else if result < *lb {
                return *lb;
            } else {
                return result;
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
                *step_prev = time;
            }
            let x = time - *step_prev;
            let result: f32 = (*a) * x.powf(2.0) + *b * x + *c;

            if result > *hb {
                return *hb;
            } else if result < *lb {
                return *lb;
            } else {
                return result;
            }
        }
    }
}

pub fn RunEvents(
    event_list: &mut Vec<Event>,
    ActiveTable: &Vec<bool>,
    CondTable: &mut Vec<bool>,
    sensor_act_values: &mut SensorActuatorValues,
    sys_time: &Instant,
    running: &mut bool
) {
    for _event in event_list {
        match _event {

            Event::SensorValue {
                event,
                sensor_target,
                sensor_prev,
                sensor_id,
                expr,
            } => {
                if ActiveTable[event.process_id] {
                    let sensor_value: f32 = getSensorValue(*sensor_id, sensor_act_values);
                    if *sensor_prev < 0.0 {
                        *sensor_prev = sensor_value;
                    }

                    match expr {
                        '>' => {
                            CondTable[event.term_conditions_id] =
                                sensor_value - *sensor_prev >= *sensor_target
                        }
                        '<' => {
                            CondTable[event.term_conditions_id] =
                                sensor_value - *sensor_prev <= *sensor_target
                        }
                        _ => {
                            error!("Invalid character {} at Events::SensorValue", expr);
                        }
                    }
                }
            }

            // Motor Control
            Event::MotorSpeedControl {
                event,
                motor_id,
                func,
            } => {
                if ActiveTable[event.process_id] {
                    let time: f32 = sys_time.elapsed().as_secs_f32();

                    setMotorPow(
                        MathFuncBasedOnTime(time, func),
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
            } => {
                if ActiveTable[event.process_id] {
                    let motor_correction: f32 =
                        ComputePID(getSensorValue(GYRO, sensor_act_values), heading, pid);
                    setMotorPow(motor_correction, LDRIVECOR, sensor_act_values);
                    setMotorPow(-motor_correction, RDRIVECOR, sensor_act_values);
                }
            }

            Event::PIDLine {
                event,
                brightness_target,
                pid,
            } => {
                if ActiveTable[event.process_id] {
                    let motor_correction: f32 = ComputePID(
                        getSensorValue(COLOURSENS, sensor_act_values),
                        brightness_target,
                        pid,
                    );
                    setMotorPow(motor_correction, LDRIVECOR, sensor_act_values);
                    setMotorPow(-motor_correction, RDRIVECOR, sensor_act_values);
                }
            }

            Event::PIDHold { event, pid } => if ActiveTable[event.process_id] {},

            // Compute and Timer
            Event::ComputeMotorStall {
                event,
                min_speed,
                buffer,
            } => if ActiveTable[event.process_id] {},

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

                            if DEBUG {
                                info!(
                                    "Timer {} over!, time passed {} s",
                                    event.process_id, time_passed
                                );
                            }
                        }
                    }
                }
            }

            Event::HaltProcessLoop { 
                event 
            } => {
                if ActiveTable[event.process_id] {
                    *running = false;
                    if DEBUG {
                        info!("Events::HaltProcessLoop terminated ProcessLoop!")
                    }
                }
            }

            _ => {}
        }
    }
}
