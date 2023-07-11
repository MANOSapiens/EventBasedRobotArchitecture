use log::{error, info, warn};

// Local modules
use super::DEBUG;
use crate::Events::{Condition, Event};
use crate::Ports::{MotorsSensors, PortDefinition, prepare_motors_sensor};
use crate::ProcessLoop::ProcessLoop;
use crate::ReadInstructions::ReadInstructions;

// Initialisation RULES
// EVERY variable with *_prev must be initialized with -1!
// the process_id 0 is allocated for usage as a none pointer!

pub fn startExecution(
    round_instructions_path: &str,
    port_definitions: PortDefinition
) {
    let mut event_list: Vec<Event> = Vec::new();
    let mut spawn_list: Vec<Condition> = Vec::new();
    let mut term_list: Vec<Condition> = Vec::new();

    let mut round_timeout: f32 = -1.0;

    ReadInstructions(round_instructions_path, &mut spawn_list, &mut event_list, &mut term_list, &mut round_timeout);

    // prepare motors sensors struct
    let motors_sensors: MotorsSensors;
    motors_sensors = prepare_motors_sensor(port_definitions);

    // prepare boolean table for listing terminated events
    let mut ActiveTable: Vec<bool> = Vec::with_capacity(event_list.len()+1);
    let mut TerminatedTable: Vec<bool> = Vec::with_capacity(event_list.len()+1);
    let mut CondTable: Vec<bool> = Vec::with_capacity(spawn_list.len() + term_list.len());


    for _ in 0..(event_list.len()+1) {
        ActiveTable.push(false);
        TerminatedTable.push(false);   
    }

    for _ in 0..(spawn_list.len() + term_list.len()) {
        CondTable.push(false);
    }

    ProcessLoop(
        spawn_list,
        event_list,
        term_list,
        motors_sensors,
        ActiveTable,
        TerminatedTable,
        CondTable,
        round_timeout
    );
}