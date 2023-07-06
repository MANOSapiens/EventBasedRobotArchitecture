// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port};
use log::{error, info, warn};
use std::time::{Duration, Instant};

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};

// Local modules
pub mod Events;
use crate::Events::{Condition, Event};

mod Ports;
use crate::Ports::{MotorsSensors, PortDefinition, prepare_motors_sensor};

mod Logger;
use crate::Logger::init_logger;

// Function Definition
mod ReadSensors;
mod RunEvents;
mod SpawnTerminateEvents;
mod Check;
mod Actuators;
mod ProcessLoop;
use crate::ProcessLoop::{ProcessLoop};

fn startExecution(
    spawn_list: &mut Vec<Condition>,
    event_list: &mut Vec<Event>,
    term_list: &mut Vec<Condition>,
    port_definitions: PortDefinition,
    log_config_file: String,
    debug: bool,
) {
    // prepare Logger
    init_logger(log_config_file);

    // prepare motors sensors struct
    let mut motors_sensors: MotorsSensors;
    motors_sensors = prepare_motors_sensor(port_definitions);

    // prepare boolean table for listing terminated events
    let mut ActiveTable: Vec<bool> = vec![];
    let mut TerminatedTable: Vec<bool> = vec![];
    let mut CondTable: Vec<bool> = vec![];


    for i in 0..(event_list.len()+1) {
        ActiveTable.push(false);
        TerminatedTable.push(false);   
    }

    for i in 0..(spawn_list.len() + term_list.len()) {
        CondTable.push(false);
    }

    ProcessLoop(
        spawn_list,
        event_list,
        term_list,
        &mut motors_sensors,
        &mut ActiveTable,
        &mut TerminatedTable,
        &mut CondTable,
        debug,
    );
}

fn main() {
    let mut EventList: Vec<Event> = Vec::new();
    let mut SpawnList: Vec<Condition> = Vec::new();
    let mut TermList: Vec<Condition> = Vec::new();

    let mut debug = true;
    let logger_config_file = String::from("log/log4rs.yaml");

    let mut port_definitions = PortDefinition {
        lDriveMotorPort: MotorPort::OutA,
        rDriveMotorPort: MotorPort::OutA,
        lToolMotorPort: MotorPort::OutA,
        rToolMotorPort: MotorPort::OutA,
        gyroSensPort: SensorPort::In1,
        colourSensPort0: SensorPort::In1,
        colourSensPort1: SensorPort::In1,
    };

    EventList.push(
        Events::Event::Timer {
            event: Events::EventID {
                process_id: 1,
                spawn_conditions_id: 0,
                term_conditions_id: 1
            },
            time: 3.0,
            time_prev: -1.0,
        }
    );

    EventList.push(
        Events::Event::Timer {
            event: Events::EventID {
                process_id: 2,
                spawn_conditions_id: 2,
                term_conditions_id: 3
            },
            time: 5.0,
            time_prev: -1.0,
        }
    );

    SpawnList.push(
        Events::Condition::StartImmediately { 
            cond: Events::CondID {
                process_id: 1,
                cond_id: 0
            } 
        }
    );

    SpawnList.push(
        Events::Condition::IsTerminated { 
            cond: Events::CondID {
                process_id: 2,
                cond_id: 2
            },
            watch_process_id: 1
        }
    );

    TermList.push(
        Events::Condition::Timer { 
            cond: Events::CondID { process_id: 1, cond_id: 1 }
        }
    );

    TermList.push(
        Events::Condition::Timer { 
            cond: Events::CondID { process_id: 2, cond_id: 3 }
        }
    );

    startExecution(
        &mut SpawnList,
        &mut EventList,
        &mut TermList,
        port_definitions,
        logger_config_file,
        debug,
    );
    // let Events::Event::Timer { start_time, .. } = &test;
    // println!("{}", start_time);
}
