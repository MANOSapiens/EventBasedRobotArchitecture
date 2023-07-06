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
use crate::ReadSensors::ReadSensors;
use crate::RunEvents::RunEvents;
use crate::SpawnTerminateEvents::{SpawnEvents, TerminateEvents};
use crate::Check::Check;

pub struct SensorActuatorValues {
    // Motor encoders with ids 0-3
    pub lDriveMotorEnc: i32,
    pub rDriveMotorEnc: i32,
    pub lToolMotorEnc: i32,
    pub rToolMotorEnc: i32,

    // Gyro, Colour with ids 4-5
    pub colourSensValue: i32,
    pub gyroAngValue: i32,

    // Motor power with ids 6-9
    pub lDriveMotorPow: i32,
    pub rDriveMotorPow: i32,
    pub lToolMotorPow: i32,
    pub rToolMotorPow: i32
}

pub fn ProcessLoop(
    spawn_list: &mut Vec<Condition>,
    event_list: &mut Vec<Event>,
    term_list: &mut Vec<Condition>,
    motors_sensors: &mut MotorsSensors,
    ActiveTable: &mut Vec<bool>,
    TerminatedTable: &mut Vec<bool>,
    CondTable: &mut Vec<bool>,
    debug: bool,
) {
    // Variable Definition
    let mut running: bool = true;
    let mut loop_counter: i32 = 0;
    let mut sys_time: Instant = Instant::now();
    let mut check_last_time: f64 = 0.0;

    // Sensor Values
    
    let mut sensor_act_values = SensorActuatorValues{
        lDriveMotorEnc: 0,
        rDriveMotorEnc: 0,
        lToolMotorEnc : 0,
        rToolMotorEnc: 0,

        // Gyro, Colour
        colourSensValue: 0,
        gyroAngValue: 0,

        // Motor power
        lDriveMotorPow: 0,
        rDriveMotorPow: 0,
        lToolMotorPow: 0,
        rToolMotorPow: 0
    };

    loop {
        // ============== MAIN LOOP =================
        // ===== Check if loop is still running =====
        if !running {
            break;
        }

        // ===== Reset Actuator Variables =====
        sensor_act_values.lDriveMotorPow = 0;
        sensor_act_values.rDriveMotorPow = 0;
        sensor_act_values.lToolMotorPow = 0;
        sensor_act_values.rToolMotorPow = 0;

        // ===== Read sensor values =====
        ReadSensors(
            motors_sensors,
            &mut sensor_act_values,
            &debug,
        );
        SpawnEvents(event_list, spawn_list, ActiveTable, TerminatedTable, CondTable);
        // Run Events
        RunEvents(event_list, &ActiveTable, CondTable, &mut sensor_act_values, &sys_time, debug);

        // Spawn and Terminate
        TerminateEvents(event_list, term_list, ActiveTable, TerminatedTable, CondTable, &mut sensor_act_values);
        

        // ===== Perform Check ======
        Check(&sys_time, &mut check_last_time, &mut loop_counter, &mut running, &debug);

        loop_counter += 1;
    }
}