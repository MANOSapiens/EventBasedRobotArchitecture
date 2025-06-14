// Import crates
extern crate ev3dev_lang_rust;


use log::{info};
use std::time::{Instant};
use std::io::Write;
use simple_moving_average::{SingleSumSMA, SumTreeSMA};




// Local modules
use super::DEBUG;
use crate::Events::{Condition, Event};
use crate::Ports::{MotorsSensors, motorsStopCoast};
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
    
    pub lDriveMotorEncRead: bool,
    pub rDriveMotorEncRead: bool,
    pub lToolMotorEncRead: bool,
    pub rToolMotorEncRead: bool,

    // Speed ids 20-24
    pub lDriveMotorSpeed: f32,
    pub rDriveMotorSpeed: f32,
    pub lToolMotorSpeed: f32,
    pub rToolMotorSpeed: f32,
    pub lDriveMotorSpeedRead: bool,
    pub rDriveMotorSpeedRead: bool,
    pub lToolMotorSpeedRead: bool,
    pub rToolMotorSpeedRead: bool,

    // These are system variables, no ids
    pub gyroAngValuePrev: f32,

    // Gyro, Colour with ids 4-5
    pub colourSensValue: f32,
    pub gyroAngValue: f32,
    pub gyroRate: f32,
    pub gyroRead: bool,

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
    pub driveMotorEncPrev: f32,

    // Motor power corrections by PIDs with ids 10-13
    pub lDriveMotorCor: f32,
    pub rDriveMotorCor: f32,
    pub lToolMotorCor: f32,
    pub rToolMotorCor: f32,

    // One Button id 18
    pub rightButton: f32,
    pub rightButtonRead: bool,
    
    // MISC
    pub currentTime: f32,
    pub timePrev: f32,
}


fn TerminateProcessLoop(sys_time: &Instant, round_summary: &mut RoundSummary, motors_sensors: &MotorsSensors, sensor_act_values: &SensorActuatorValues) {
    round_summary.wall_time = sys_time.elapsed().as_secs();
    round_summary.mean_loop_time = sys_time.elapsed().as_secs_f32() / (round_summary.loop_count as f32);
    round_summary.mean_f =  (1.0 / round_summary.mean_loop_time) as u32;

    
    info!("TERMINATED PROCESS LOOP");
    info!("Note down some stats about this round ...");
    
    motorsStopCoast(motors_sensors);

    
    info!("=========== ROUND SUMMARY ===========");
    info!("Loop count: {}", round_summary.loop_count);
    info!("Wall time: {}s", round_summary.wall_time);
    info!("Average loop time(higher means worse): {}s", round_summary.mean_loop_time as f32);
    info!("Average loop frequency: {}Hz", round_summary.mean_f);
    info!("Loop time maximum (higher means worse): {}s", round_summary.max_loop_time as f32);
    
}

pub fn ProcessLoop<W: Write>(
    spawn_list: Vec<Condition>,
    mut event_list: Vec<Event>,
    term_list: Vec<Condition>,
    motors_sensors: MotorsSensors,
    ActiveTable: &mut Vec<bool>,
    TerminatedTable: &mut Vec<bool>,
    CondTable: &mut Vec<bool>,
    round_timeout: f32,
    mut wtr: csv::Writer<W>,
) {
    // Variable Definition
    let mut running: bool = true;
    let sys_time: Instant = Instant::now();
    let mut round_summary: RoundSummary = RoundSummary {
        wall_time: 0,
        max_loop_time: 0.0,
        mean_loop_time: 0.0,
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
        lDriveMotorEncRead: true,
        rDriveMotorEncRead: true,
        lToolMotorEncRead: true,
        rToolMotorEncRead: true,
        
        // SPEEDs
        lDriveMotorSpeed: 0.0,
        rDriveMotorSpeed: 0.0,
        lToolMotorSpeed: 0.0,
        rToolMotorSpeed: 0.0,
        lDriveMotorSpeedRead: false,
        rDriveMotorSpeedRead: false,
        lToolMotorSpeedRead: false,
        rToolMotorSpeedRead: false,

        // Gyro, Colour
        colourSensValue: 0.0,
        gyroAngValue: 0.0,
        gyroRate: 0.0,
        gyroAngValuePrev: 0.0,
        gyroRead: true,

        // Motor power
        lDriveMotorPow: 0.0,
        rDriveMotorPow: 0.0,
        lToolMotorPow: 0.0,
        rToolMotorPow: 0.0,

        lDriveMotorPowPrev: 0.0,
        rDriveMotorPowPrev: 0.0,
        lToolMotorPowPrev: 0.0,
        rToolMotorPowPrev: 0.0,
        driveMotorEncPrev: 0.0,

        // Motor correction
        lDriveMotorCor: 0.0,
        rDriveMotorCor: 0.0,
        lToolMotorCor: 0.0,
        rToolMotorCor: 0.0,

        //Buttons
        rightButton: 0.0,
        rightButtonRead: true,
        currentTime: 0.0,
        timePrev: 0.0
    };

    ReadSensors(
        &motors_sensors,
        &mut sensor_act_values,
        &sys_time,
        &mut wtr,
    );

    sensor_act_values.gyroAngValuePrev = sensor_act_values.gyroAngValue;
    sensor_act_values.gyroRead = true;

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
            &mut wtr,
        );
        
        // ===== Run Events =====
        RunEvents(&mut event_list, ActiveTable, CondTable, &mut sensor_act_values, &sys_time, &mut running);
        
        // ===== Spawn and Terminate =====
        TerminateEvents( &term_list, ActiveTable, TerminatedTable, CondTable, &mut sensor_act_values);
        SpawnEvents(&spawn_list, ActiveTable, TerminatedTable, CondTable, &mut sensor_act_values);

        // ===== Write computed values to actuators =====
        writeToActuators(&motors_sensors, &mut sensor_act_values);
        
        // ===== Perform Check ======
        Check(&mut round_summary, &sensor_act_values, &round_timeout);
    }

    // reset ActiveTable CondTable and TerminatedTable to false
    for i in 0..ActiveTable.len() {
        ActiveTable[i] = false;
    }
    ActiveTable[0] = true;
    for i in 0..TerminatedTable.len() {
        TerminatedTable[i] = false;
    }
    for i in 0..CondTable.len() {
        CondTable[i] = false;
    }
}