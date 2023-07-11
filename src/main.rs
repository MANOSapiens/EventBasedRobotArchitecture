// Import crates
extern crate ev3dev_lang_rust;

use log::{error, info, warn};
use std::time::{Duration, Instant};

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};

// Local modules
pub mod Events;
use crate::Events::{Condition, Event};

mod Ports;
use crate::Ports::PortDefinition;

mod Logger;
use crate::Logger::init_logger;

// Function Definition
mod Actuators;
mod Check;
mod PID;
mod ProcessLoop;
mod ReadInstructions;
mod ReadSensors;
mod RunEvents;
mod SpawnTerminateEvents;
mod StartExecution;
pub mod consts;

use crate::consts::*;
use crate::StartExecution::startExecution;

fn main() {
    // Initialize logger
    let logger_config_file = String::from("log/log4rs.yaml");
    init_logger(logger_config_file);

    let port_definitions = PortDefinition {
        lDriveMotorPort: MotorPort::OutB,
        rDriveMotorPort: MotorPort::OutC,
        lToolMotorPort: MotorPort::OutD,
        rToolMotorPort: MotorPort::OutA,
        gyroSensPort: SensorPort::In1,
        colourSensPort: SensorPort::In4,
    };

    startExecution("timer5s.json", port_definitions);
}
