extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::Ev3Result;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{SensorPort};

fn main() {
    struct EventID {
        ID: i16,
        spawnConditionsID: i16,
        termConditionsID: i16
    }

    // =====================
    // =       Helper      =
    // =====================

    // Function types
    enum FuncTypes {
        ConstFunc {
            c: f32
        },

        LinearFunc {
            m: f32,
            e: f32
        },
        
        QuadFunc {
            a: f32,
            n: i8,
            e: f32
        }
    }

    struct PID {
        p: f32,
        i: f32,
        d: f32,
        maxI: f32,
        sumI: f32,
        prevE: f32
    }

    // =====================
    // =     Processes     =
    // =====================

    // THE PLACEHOLDER PROCESS
    enum Event {
        None,
    
        Placeholder {
            event: EventID
        },

        // Motor Processses

        MotorSpeedControl {
            event: EventID,
            motorPort: MotorPort,
            accelFunc: FuncTypes,
        },

        // PID Processes
        
        PIDGyro {
            event: EventID,
            gyroPort: SensorPort,
            heading: i16,
            pid: PID
        },

        PIDLine {
            event: EventID,
            colorSensPort: SensorPort,
            brightnessTarget: i16,
            pid: PID
        },
        

        // Compute Processes

        Timer {
            event: EventID,
            time: f32, // time in seconds
            startTime: f32,
        },

        ComputeMotorStall {
            event: EventID,
            minSpeed: i8,
            buffer: [i8; 10]
        }
    }

    let mut EventList: Vec<Event> = Vec::new();

    EventList.push(
        Event::Placeholder {
            event: EventID {
                    ID: 0,
                    spawnConditionsID: 0,
                    termConditionsID: 0
                }
        }
    );

    println!("sss");
}