use ev3dev_lang_rust::Led;

// Local modules
use super::DEBUG;
use crate::Events::{Condition, Event};
use crate::Logger::logHeaderCSV;
use crate::Ports::{MotorsSensors, PortDefinition, prepare_motors_sensor, setSpeedPID};
use crate::ProcessLoop::ProcessLoop;
use crate::ReadInstructions::ReadInstructions;

// Initialisation RULES
// EVERY variable with *_prev must be initialized with -1!
// the process_id 0 is allocated for usage as a none pointer!

pub fn startExecution<'a>(
    round_instructions_path: &'a str,
    port_definitions: &'a PortDefinition,
    ActiveTable: &'a mut Vec<bool>,
    TerminatedTable: &'a mut Vec<bool>,
    CondTable: &'a mut Vec<bool>,
) -> Result<i8, &'a str>{
    let mut event_list: Vec<Event> = Vec::new();
    let mut spawn_list: Vec<Condition> = Vec::new();
    let mut term_list: Vec<Condition> = Vec::new();
    let led: Led = Led::new().expect("cant initialize led!");
    led.set_color(Led::COLOR_RED).expect("cant set led color!");

    let mut name: String = String::from("");

    let mut round_timeout: f32 = -1.0;
    let mut speed_p: f32 = 0.0;
    let mut speed_i: f32 = 0.0;
    let mut speed_d: f32 = 0.0;
    let mut file_forward: i8 = 1;

    ReadInstructions(round_instructions_path, &mut spawn_list, &mut event_list, &mut term_list, &mut round_timeout, &mut speed_p, &mut speed_i, &mut speed_d, &mut name, &mut file_forward);
    let mut wtr = csv::Writer::from_path(format!("records/{name}.csv")).expect("cant initialize csv writer!");
    
    if DEBUG {
        logHeaderCSV(&mut wtr).expect("cant write header to CSV file!");
    }

    // prepare motors sensors struct
    
    let motors_sensors: MotorsSensors = match prepare_motors_sensor(&port_definitions, speed_p, speed_i, speed_d){
        Ok(motors_sensors)  => motors_sensors,
        Err(e) => return Err(e),
    };

    setSpeedPID(&motors_sensors, speed_p, speed_i, speed_d);
    
    // check lenghts of tables
    // compare to event_list, spawn_list, term_list lengths
    if ActiveTable.len() < event_list.len() ||  TerminatedTable.len() < event_list.len() || CondTable.len() < spawn_list.len() + term_list.len(){
        let mut ActiveTable: Vec<bool> = vec![false; event_list.len()+1];
        ActiveTable[0] = true;
        let mut TerminatedTable: Vec<bool> = vec![false; event_list.len()+1];
        let mut CondTable: Vec<bool> = vec![false; spawn_list.len() + term_list.len()];
    }
    
    
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

    led.set_color(Led::COLOR_GREEN).expect("cant set led color!");
    Ok(file_forward)
}