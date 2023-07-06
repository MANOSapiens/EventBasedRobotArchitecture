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
use crate::ProcessLoop::SensorActuatorValues;
use crate::ReadSensors::getSensorValue;


pub fn RunEvents(event_list: &mut Vec<Event>, ActiveTable: &Vec<bool>, CondTable: &mut Vec<bool>, sensor_act_values: &mut SensorActuatorValues, sys_time: &Instant, debug: bool) {
    for _event in event_list {
        match _event {
            // Helper
            Event::None => {
                println!("A None event!");
            }

            Event::Placeholder { event } => {
                println!("{}", event.process_id);
            }

            Event::SensorValue { event, sensor_target, sensor_prev, sensor_id, expr, is_set } => {
                if ActiveTable[event.process_id] {
                    let mut sensor_value = getSensorValue(sensor_id, sensor_act_values);

                    if !*is_set {
                        *sensor_prev = sensor_value;
                        *is_set = true;
                    }

                    match expr {
                        '>' => {CondTable[event.term_conditions_id] = sensor_value - *sensor_prev > *sensor_target},
                        '<' => {CondTable[event.term_conditions_id] = sensor_value - *sensor_prev < *sensor_target},
                        _ => {
                            error!("Invalid character {} at Events::SensorValue", expr);
                        }
                    }
                }
            }

            // Motor Control
            Event::MotorSpeedControl {
                event: EventID,
                motor_id,
                accel_func: FuncTypes,
            } => {

            }

            // PID Control
            Event::PIDGyro {
                event,
                gyro_prev,
                heading,
                pid,
            } => {

            }

            Event::PIDLine {
                event,
                brightness_target,
                pid,
            } => {

            }

            // Compute and Timer
            Event::ComputeMotorStall {
                event,
                min_speed,
                buffer,
            } => {

            }

            Event::Timer {
                event,
                time,
                time_prev,
            } => {
                if ActiveTable[event.process_id] {
                    
                    if *time_prev == -1.0 {
                        *time_prev = sys_time.elapsed().as_secs_f64();
                    } else {
                        let time_passed = sys_time.elapsed().as_secs_f64() - *time_prev;

                        
                        if time_passed >= *time {
                            CondTable[event.term_conditions_id] = true;
                            info!("Timer over!, time passed {}", time_passed)
                        }
                    }
                }
            }
        }
    }
}