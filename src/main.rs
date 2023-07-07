// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port};
use log::{error, info, warn};
use std::time::{Duration, Instant};

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};

use std::fs;

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
mod PID;
mod SpawnTerminateEvents;
mod Check;
mod Actuators;
mod ProcessLoop;
use crate::ProcessLoop::{ProcessLoop};

static DEBUG: bool = true;

static LDRIVEENC: i8 = 0;
static RDRIVEENC: i8 = 1;
static LTOOLENC: i8 = 2;
static RTOOLENC: i8 = 3;
static COLOURSENS: i8 = 4;
static GYRO: i8 = 5;
static LDRIVEPOW: i8 = 6;
static RDRIVEPOW: i8 = 7;
static LTOOLPOW: i8 = 8;
static RTOOLPOW: i8 = 9;
static LDRIVECOR: i8 = 10;
static RDRIVECOR: i8 = 11;
static LTOOLCOR: i8 = 12;
static RTOOLCOR: i8 = 13;

static EVENT_NONE: i8 = 0;
static EVENT_PLACEHOLDER: i8 = 1;
static EVENT_SENSORVALUE: i8 = 2;
static EVENT_MOTORSPEEDCONTROL: i8 = 3;
static EVENT_PIDGYRO: i8 = 4;
static EVENT_PIDLINE: i8 = 5;
static EVNET_PIDHOLD: i8 = 6;
static EVENT_TIMER: i8 = 7;
static EVENT_COMPUTEMOTORSTALL: i8 = 8;


// Initialisation RULES
// EVERY variable with *_prev must be initialized with -1!
// the process_id 0 is allocated for usage as a none pointer!

fn startExecution(
    spawn_list: &mut Vec<Condition>,
    event_list: &mut Vec<Event>,
    term_list: &mut Vec<Condition>,
    port_definitions: PortDefinition,
    log_config_file: String
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
        &mut CondTable
    );
}

fn main() {
    let mut EventList: Vec<Event> = Vec::new();
    let mut SpawnList: Vec<Condition> = Vec::new();
    let mut TermList: Vec<Condition> = Vec::new();

    let file = fs::File::open("text.json")
        .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
        .unwrap();
    
    println!("{}",json.get("PhoneNumbers").expect("msg").get(0).expect("msg"));

    let logger_config_file = String::from("log/log4rs.yaml");

    let mut port_definitions = PortDefinition {
        lDriveMotorPort: MotorPort::OutB,
        rDriveMotorPort: MotorPort::OutC,
        lToolMotorPort: MotorPort::OutD,
        rToolMotorPort: MotorPort::OutA,
        gyroSensPort: SensorPort::In1,
        colourSensPort0: SensorPort::In4,
        colourSensPort1: SensorPort::In2,
    };

    /* EventList.push(
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
        Events::Event::MotorSpeedControl {
            event: Events::EventID {
                process_id: 2,
                spawn_conditions_id: 2,
                term_conditions_id: 3
            },
            motor_id: LDRIVEPOW,
            func: Events::FuncTypes::LinearFunc { m: 0.1, e: 0.0, lb: 0.0, hb: 1.0, step_prev: -1.0 }
        }
    );

    EventList.push(
        Events::Event::SensorValue {
            event: Events::EventID {
                process_id: 3,
                spawn_conditions_id: 4,
                term_conditions_id: 5
            },
            sensor_target: 0.5, 
            sensor_prev: 0.0, 
            sensor_id: 6, 
            expr: '>'
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
        Events::Condition::StartImmediately { 
            cond: Events::CondID {
                process_id: 3,
                cond_id: 4
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
        Events::Condition::IsTerminated{ 
            cond: Events::CondID { process_id: 2, cond_id: 3 },
            watch_process_id: 3
        }
    );

    TermList.push(
        Events::Condition::SensorValue { 
            cond: Events::CondID { 
                process_id: 3, 
                cond_id: 5 
            }
        }
    ); */

    startExecution(
        &mut SpawnList,
        &mut EventList,
        &mut TermList,
        port_definitions,
        logger_config_file
    );
    // let Events::Event::Timer { start_time, .. } = &test;
    // println!("{}", start_time);
}
