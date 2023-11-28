

// Local modules
use super::DEBUG;
use crate::Events::{Condition, Event};
use crate::Logger::logHeaderCSV;
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

    let mut name: String = String::from("");

    let mut round_timeout: f32 = -1.0;

    ReadInstructions(round_instructions_path, &mut spawn_list, &mut event_list, &mut term_list, &mut round_timeout, &mut name);
    let mut wtr = csv::Writer::from_path(format!("records/{name}.csv")).expect("cant initialize csv writer!");
    
    if DEBUG {
        logHeaderCSV(&mut wtr).expect("cant write header to CSV file!");
    }

    // prepare motors sensors struct
    
    let motors_sensors: MotorsSensors = prepare_motors_sensor(port_definitions);

    // prepare boolean table for listing terminated events
    let mut ActiveTable: Vec<bool> = vec![false; event_list.len()+1];
    ActiveTable[0] = true;
    let mut TerminatedTable: Vec<bool> = vec![false; event_list.len()+1];
    let mut CondTable: Vec<bool> = vec![false; spawn_list.len() + term_list.len()];
    

    ProcessLoop(
        spawn_list,
        event_list,
        term_list,
        motors_sensors,
        ActiveTable,
        TerminatedTable,
        CondTable,
        round_timeout,
        wtr,
    );
}