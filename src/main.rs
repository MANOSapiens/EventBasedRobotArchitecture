// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port};
use log::{error, info, warn};
use std::time::{Duration, Instant};

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};

// Local modules
mod Events;
use crate::Events::{Condition, Event};

mod Ports;
use crate::Ports::{MotorsSensors, PortDefinition};

mod Logger;
use crate::Logger::init_logger;

// Function Definition
fn ReadSensors(
    motors_sensors: &mut MotorsSensors,
    lDriveMotorEnc: &mut i32,
    rDriveMotorEnc: &mut i32,
    lToolMotorEnc: &mut i32,
    rToolMotorEnc: &mut i32,
    colourSensValue: &mut i8,
    gyroAngValue: &mut i32,
    debug: &bool,
) {
}

fn Check(time_now: Instant, loop_counter: &mut i32,debug: &bool) -> Instant {
    let elapsed: f64 = time_now.elapsed().as_secs_f64();
    let mut time_now: Instant = Instant::now();
    println!("{}", elapsed);
    time_now
}

fn ProcessLoop(
    spawn_list: &mut Vec<Condition>,
    event_list: &mut Vec<Event>,
    term_list: &mut Vec<Condition>,
    motors_sensors: &mut MotorsSensors,
    debug: bool,
) {
    // Variable Definition
    let mut running = true;
    let mut loop_counter: i32 = 0;
    let mut time_now = Instant::now();

    // Sensor Values
    // Motor encoders
    let mut lDriveMotorEnc: i32 = 0;
    let mut rDriveMotorEnc: i32 = 0;
    let mut lToolMotorEnc: i32 = 0;
    let mut rToolMotorEnc: i32 = 0;

    // Gyro, Colour
    let mut colourSensValue: i8 = 0;
    let mut gyroAngValue: i32 = 0;

    // Motor power
    let mut lDriveMotorPow: i8 = 0;
    let mut rDriveMotorPow: i8 = 0;
    let mut lToolMotorPow: i8 = 0;
    let mut rToolMotorPow: i8 = 0;

    loop {
        // ============== MAIN LOOP =================
        // ===== Check if loop is still running =====
        if !running {
            break;
        }

        // ===== Reset Variables =====
        lDriveMotorPow = 0;
        rDriveMotorPow = 0;
        lToolMotorPow = 0;
        rToolMotorPow = 0;

        // ===== Read sensor values =====
        ReadSensors(
            motors_sensors,
            &mut lDriveMotorEnc,
            &mut rDriveMotorEnc,
            &mut lToolMotorEnc,
            &mut rToolMotorEnc,
            &mut colourSensValue,
            &mut gyroAngValue,
            &debug,
        );

        // ===== Perform Check ======
        time_now = Check(time_now, &mut loop_counter, &debug);

        running = false;
    }
}

fn prepare_motors_sensor(port_definitions: PortDefinition) -> MotorsSensors {
    // Try to init all motors and sensors
    // Panics (throws error) if not available

    /* let mut motors_sensors = MotorsSensors {
        lDriveMotor: LargeMotor::get(port_definitions.lDriveMotorPort)
            .expect("failed to load motor or sensor"),
        rDriveMotor: LargeMotor::get(port_definitions.rDriveMotorPort)
            .expect("failed to load motor or sensor"),
        lToolMotor: MediumMotor::get(port_definitions.lToolMotorPort)
            .expect("failed to load motor or sensor"),
        rToolMotor: MediumMotor::get(port_definitions.rToolMotorPort)
            .expect("failed to load motor or sensor"),
        gyroSens: GyroSensor::get(port_definitions.gyroSensPort)
            .expect("failed to load motor or sensor"),
        colourSens0: ColorSensor::get(port_definitions.colourSensPort0)
            .expect("failed to load motor or sensor"),
        colourSens1: ColorSensor::get(port_definitions.colourSensPort1)
            .expect("failed to load motor or sensor"),
    }; */

    let mut motors_sensors = MotorsSensors {

    };

    motors_sensors
}

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

    ProcessLoop(
        spawn_list,
        event_list,
        term_list,
        &mut motors_sensors,
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

    EventList.push(Events::Event::Timer {
        event: Events::EventID {
            process_id: 0,
            spawn_conditions_id: 0,
            term_conditions_id: 0,
            active: false,
        },
        time: 0.0,
        time_prev: 0.0,
    });

    EventList.push(Events::Event::Placeholder {
        event: Events::EventID {
            process_id: 1,
            spawn_conditions_id: 1,
            term_conditions_id: 1,
            active: false,
        },
    });

    for _event in &mut EventList {
        match _event {
            // Helper
            Event::None => {
                println!("A None event!");
            }

            Event::Placeholder { event } => {
                event.process_id = 5;
                println!("{}", event.process_id);
            }

            // Motor Control
            Event::MotorSpeedControl {
                event: EventID,
                motor_id,
                accel_func: FuncTypes,
            } => {}

            // PID Control
            Event::PIDGyro {
                event,
                gyro_prev,
                heading,
                pid,
            } => {}

            Event::PIDLine {
                event,
                brightness_target,
                pid,
            } => {}

            // Compute and Timer
            Event::ComputeMotorStall {
                event,
                min_speed,
                buffer,
            } => {}

            Event::Timer {
                event,
                time,
                time_prev,
            } => {}
        }
    }

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
