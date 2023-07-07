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
mod ReadInstructions;
mod ReadSensors;
mod RunEvents;
mod PID;
mod SpawnTerminateEvents;
mod Check;
mod Actuators;
mod ProcessLoop;

use crate::ReadInstructions::ReadInstructions;
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
    port_definitions: PortDefinition
) {
    

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

    // Initialize logger
    let logger_config_file = String::from("log/log4rs.yaml");
    init_logger(logger_config_file);

    let mut port_definitions = PortDefinition {
        lDriveMotorPort: MotorPort::OutB,
        rDriveMotorPort: MotorPort::OutC,
        lToolMotorPort: MotorPort::OutD,
        rToolMotorPort: MotorPort::OutA,
        gyroSensPort: SensorPort::In1,
        colourSensPort0: SensorPort::In4,
        colourSensPort1: SensorPort::In2,
    };

    ReadInstructions("text.json", &mut SpawnList, &mut EventList, &mut TermList);

    startExecution(
        &mut SpawnList,
        &mut EventList,
        &mut TermList,
        port_definitions
    );
    // let Events::Event::Timer { start_time, .. } = &test;
    // println!("{}", start_time);
}
