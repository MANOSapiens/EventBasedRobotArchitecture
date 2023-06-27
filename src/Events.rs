extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::Ev3Result;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{SensorPort, Sensor};

pub struct EventID {
    pub process_id: i16,
    pub spawn_conditions_id: i16,
    pub term_conditions_id: i16,
    pub active: bool
}

// =====================
// =       Helper      =
// =====================

// Function types
pub enum FuncTypes {
    ConstFunc {
        c: f32,
        step: i32
    },

    LinearFunc {
        m: f32,
        e: f32,
        step: i32
    },
    
    QuadFunc {
        a: f32,
        n: i8,
        e: f32,
        step: i32
    }
}

pub struct PID {
    pub p: f32,
    pub i: f32,
    pub d: f32,
    pub max_i: f32,
    pub sum_i: f32,
    pub prev_e: f32
}

// =====================
// =     Processes     =
// =====================

// THE PLACEHOLDER PROCESS
pub enum Event {
    None,

    Placeholder {
        event: EventID
    },

    // Motor Processses

    MotorSpeedControl {
        event: EventID,
        motor_id: i8,
        accel_func: FuncTypes,
    },

    // PID Processes
    
    PIDGyro {
        event: EventID,
        gyro_prev: i32,
        heading: i16,
        pid: PID
    },

    PIDLine {
        event: EventID,
        brightness_target: i16,
        pid: PID
    },
    

    // Compute Processes
    Timer {
        event: EventID,
        time: f32, // time in seconds
        time_prev: f32,
    },

    ComputeMotorStall {
        event: EventID,
        min_speed: i8,
        buffer: [i8; 10]
    }
}

// =====================
// =    Conditions     =
// =====================
pub struct CondID {
    pub process_id: i16,
    pub cond_id: i16,
    pub active: bool,
    pub result: bool
}

pub enum Condition {
    None,

    IsTerminated {
        cond: CondID,
        watch_process_id: i16,
        
    },

    And {
        cond: CondID,
        watch_cond_id0: i16,
        watch_cond_id1: i16
    },

    Or {
        cond: CondID,
        watch_cond_id0: i16,
        watch_cond_id1: i16
    },

    Not {
        cond: CondID,
        watch_cond_id: i16
    },

    StartImmediately {
        cond: CondID,
    },

    StopImmediately {
        cond: CondID,
    },

    MotorRotValue {
        cond: CondID,
        motor_id: i8,
        trigger_value: i32,
        prev_value: i32
    },

    SensorValue {
        cond: CondID,
        sensor_id: i8,
        trigger_value: f32,
        prev_value: f32
    },

    Timer {
        cond: CondID,
        abs_trigger_time: f32,
        trigger_value: f32,
    },
}