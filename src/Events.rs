extern crate ev3dev_lang_rust;

use super::DEBUG;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{Sensor, SensorPort};
use ev3dev_lang_rust::Ev3Result;

pub struct EventID {
    pub process_id: usize,
    pub spawn_conditions_id: usize,
    pub term_conditions_id: usize,
}

// =====================
// =       Helper      =
// =====================

// Function types
pub enum FuncTypes {
    ConstFunc {
        c: f32,
    },

    LinearFunc {
        m: f32,
        e: f32,
        lb: f32,
        hb: f32,
        step_prev: f32,
    },

    QuadFunc {
        a: f32,
        b: f32,
        c: f32,
        lb: f32,
        hb: f32,
        step_prev: f32,
    },
}

pub struct PID {
    pub p: f32,
    pub i: f32,
    pub d: f32,
    pub max_i: f32,
    pub sum_i: f32,
    pub prev_e: f32,
}

// =====================
// =     Processes     =
// =====================

// THE PLACEHOLDER PROCESS
pub enum Event {
    None,

    Placeholder {
        event: EventID,
    },

    SensorValue {
        event: EventID,
        sensor_target: f32,
        sensor_prev: f32,
        sensor_id: i8,
        expr: char,
    },

    // Motor Processses
    // motor_id
    // 0 -> left drive motor
    // 1 -> right drive motor
    // 2 -> left tool motor
    // 3 -> right tool motor
    MotorSpeedControl {
        event: EventID,
        motor_id: i8,
        func: FuncTypes,
    },

    // PID Processes
    PIDGyro {
        event: EventID,
        heading: f32,
        pid: PID,
    },

    PIDLine {
        event: EventID,
        brightness_target: f32,
        pid: PID,
    },

    PIDHold {
        event: EventID,
        pid: PID,
    },

    // Compute Processes
    Timer {
        event: EventID,
        time: f32,
        time_prev: f32,
    },

    ComputeMotorStall {
        event: EventID,
        min_speed: f32,
        buffer: [f32; 10],
    },

    // System Events
    HaltProcessLoop {
        event: EventID
    }
}

// =====================
// =    Conditions     =
// =====================
pub struct CondID {
    pub process_id: usize,
    pub cond_id: usize,
}

pub enum Condition {
    Placeholder {
        cond: CondID
    },

    IsTerminated {
        cond: CondID,
        watch_process_id: usize,
    },

    And {
        cond: CondID,
        watch_cond_id0: usize,
        watch_cond_id1: usize,
    },

    Or {
        cond: CondID,
        watch_cond_id0: usize,
        watch_cond_id1: usize,
    },

    Not {
        cond: CondID,
        watch_cond_id: usize,
    },

    StartImmediately {
        cond: CondID,
    },

    StopImmediately {
        cond: CondID,
    },

    SensorValue {
        cond: CondID,
    },

    Timer {
        cond: CondID,
    },
}
