// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port};
use log::{error, info, warn};
use std::time::{Duration, Instant};

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, GyroSensor, SensorPort};

// Local modules
use super::DEBUG;
use crate::Events::{Condition, Event};
use crate::Ports::{MotorsSensors, PortDefinition, motorsStopCoast};
use crate::ReadSensors::{resetAll, ReadSensors};
use crate::RunEvents::RunEvents;
use crate::SpawnTerminateEvents::{SpawnEvents, TerminateEvents};
use crate::Actuators::writeToActuators;
use crate::Check::{Check, RoundSummary};

pub struct SensorActuatorValues {
    // Motor encoders with ids 0-3
    pub lDriveMotorEnc: f32,
    pub rDriveMotorEnc: f32,
    pub lToolMotorEnc: f32,
    pub rToolMotorEnc: f32,

    // These are system variables, no ids
    pub lDriveMotorEncPrev: f32,
    pub rDriveMotorEncPrev: f32,
    pub lToolMotorEncPrev: f32,
    pub rToolMotorEncPrev: f32,

    // Gyro, Colour with ids 4-5
    pub colourSensValue: f32,
    pub gyroAngValue: f32,

    // Motor power with ids 6-9
    pub lDriveMotorPow: f32,
    pub rDriveMotorPow: f32,
    pub lToolMotorPow: f32,
    pub rToolMotorPow: f32,

    // These are system variables, no ids
    pub lDriveMotorPowPrev: f32,
    pub rDriveMotorPowPrev: f32,
    pub lToolMotorPowPrev: f32,
    pub rToolMotorPowPrev: f32,

    // Motor power corrections by PIDs with ids 10-13
    pub lDriveMotorCor: f32,
    pub rDriveMotorCor: f32,
    pub lToolMotorCor: f32,
    pub rToolMotorCor: f32,

    // Motor speed with ids 14-17
    pub lDriveMotorSpeed: f32,
    pub rDriveMotorSpeed: f32,
    pub lToolMotorSpeed: f32,
    pub rToolMotorSpeed: f32,

    // One Button id 18
    pub centerButton: f32,
}


fn TerminateProcessLoop(sys_time: &Instant, round_summary: &mut RoundSummary, motors_sensors: &MotorsSensors, sensor_act_values: &SensorActuatorValues) {
    round_summary.wall_time = sys_time.elapsed().as_secs();
    round_summary.mean_loop_time = sys_time.elapsed().as_secs_f64() / (round_summary.loop_count as f64);
    round_summary.mean_f =  (1.0 / round_summary.mean_loop_time) as u32;

    if DEBUG {
        info!("TERMINATED PROCESS LOOP");
        info!("Note down some stats about this round ...");
    }
    
    motorsStopCoast(motors_sensors);

    round_summary.total_travelled_distance = ((sensor_act_values.lDriveMotorEnc + sensor_act_values.rDriveMotorEnc)/2.0) as i32;

    if DEBUG {
        info!("=========== ROUND SUMMARY ===========");
        info!("Loop count: {}", round_summary.loop_count);
        info!("Wall time: {}s", round_summary.wall_time);
        info!("Average loop time(higher means worse): {}s", round_summary.mean_loop_time as f32);
        info!("Average loop frequency: {}Hz", round_summary.mean_f);
        info!("Loop time maximum (higher means worse): {}s", round_summary.max_loop_time as f32);
        
        info!("Total travelled distance in motor degrees: {}", round_summary.total_travelled_distance);
    }
}

pub fn ProcessLoop(
    spawn_list: Vec<Condition>,
    mut event_list: Vec<Event>,
    term_list: Vec<Condition>,
    motors_sensors: MotorsSensors,
    mut ActiveTable: Vec<bool>,
    mut TerminatedTable: Vec<bool>,
    mut CondTable: Vec<bool>,
    round_timeout: f32
) {
    // Variable Definition
    let mut running: bool = true;
    let sys_time: Instant = Instant::now();
    let mut check_last_time: f64 = 0.0;
    let mut read_sensor_last_time: f32 = 0.0;
    let mut round_summary: RoundSummary = RoundSummary {
        wall_time: 0,
        max_loop_time: 0.0,
        mean_loop_time: 0.0,
        total_travelled_distance: 0,
        loop_count: 0,
        mean_f: 0
    };

    // Sensor Values
    resetAll(&motors_sensors);

    let mut sensor_act_values = SensorActuatorValues{
        lDriveMotorEnc: 0.0,
        rDriveMotorEnc: 0.0,
        lToolMotorEnc : 0.0,
        rToolMotorEnc: 0.0,

        lDriveMotorEncPrev: 0.0,
        rDriveMotorEncPrev: 0.0,
        lToolMotorEncPrev: 0.0,
        rToolMotorEncPrev: 0.0,

        // Gyro, Colour
        colourSensValue: 0.0,
        gyroAngValue: 0.0,

        // Motor power
        lDriveMotorPow: 0.0,
        rDriveMotorPow: 0.0,
        lToolMotorPow: 0.0,
        rToolMotorPow: 0.0,

        lDriveMotorPowPrev: 0.0,
        rDriveMotorPowPrev: 0.0,
        lToolMotorPowPrev: 0.0,
        rToolMotorPowPrev: 0.0,

        // Motor correction
        lDriveMotorCor: 0.0,
        rDriveMotorCor: 0.0,
        lToolMotorCor: 0.0,
        rToolMotorCor: 0.0,

        // Motor speed
        lDriveMotorSpeed: 0.0,
        rDriveMotorSpeed: 0.0,
        lToolMotorSpeed: 0.0,
        rToolMotorSpeed: 0.0,

        //Buttons
        centerButton: 0.0,
    };

    loop {
        // ============== MAIN LOOP =================
        // ===== Check if loop is still running =====
        if !running {
            TerminateProcessLoop(&sys_time, &mut round_summary, &motors_sensors, &sensor_act_values);
            break;
        }

        // ===== Read sensor values =====
        ReadSensors(
            &motors_sensors,
            &mut sensor_act_values,
            &sys_time,
            &mut read_sensor_last_time
        );
        
        // ===== Run Events =====
        RunEvents(&mut event_list, &ActiveTable, &mut CondTable, &mut sensor_act_values, &sys_time, &mut running);

        // ===== Spawn and Terminate =====
        TerminateEvents(&mut event_list, &term_list, &mut ActiveTable, &mut TerminatedTable, &mut CondTable, &mut sensor_act_values);
        SpawnEvents(&mut event_list, &spawn_list, &mut ActiveTable, &mut TerminatedTable, &mut CondTable);

        // ===== Write computed values to actuators =====
        writeToActuators(&motors_sensors, &mut sensor_act_values);
        
        // ===== Perform Check ======
        Check(&sys_time, &mut check_last_time, &mut round_summary, &sensor_act_values, &round_timeout);
    }
}