// Import crates
extern crate ev3dev_lang_rust;

// Local modules
use super::DEBUG;
use crate::Events::{Condition, Event, CondID};
use crate::RunEvents::setReadSensor;
use crate::ProcessLoop::SensorActuatorValues;



fn setVarSpawn(result: bool, cond: &CondID, ActiveTable: &mut Vec<bool>, CondTable: &mut Vec<bool>, sensor_act_values: &mut SensorActuatorValues){
    CondTable[cond.cond_id] = result;
    if result {
        if cond.process_id != 0 {
            ActiveTable[cond.process_id] = true;
            setReadSensor(cond.sensor_needed, sensor_act_values);
        }
    }
}


fn setVarTerm(result: bool, cond: &CondID, ActiveTable: &mut Vec<bool>, TerminatedTable: &mut Vec<bool>, CondTable: &mut Vec<bool>){
    CondTable[cond.cond_id] = result;
    if result && cond.process_id != 0 {
        ActiveTable[cond.process_id] = false;
        TerminatedTable[cond.process_id] = true;
        
    }
}


pub fn SpawnEvents(spawn_list: &Vec<Condition>, ActiveTable: &mut Vec<bool>, TerminatedTable: & Vec<bool>, CondTable: &mut Vec<bool>, sensor_act_values: &mut SensorActuatorValues) {
    for _condition in spawn_list {
        match _condition {
            Condition::IsTerminated { cond, watch_process_id } => {
                if !CondTable[cond.cond_id] {
                    setVarSpawn(TerminatedTable[*watch_process_id], cond, ActiveTable, CondTable, sensor_act_values);
                } 
            }

            Condition::And { cond, watch_cond_id0, watch_cond_id1 } => {
                if !CondTable[cond.cond_id]{
                    setVarSpawn(CondTable[*watch_cond_id0] && CondTable[*watch_cond_id1], cond, ActiveTable, CondTable, sensor_act_values);
                }
            }

            Condition::Or { cond, watch_cond_id0, watch_cond_id1 } => {
                if !CondTable[cond.cond_id]{
                    setVarSpawn(CondTable[*watch_cond_id0] || CondTable[*watch_cond_id1], cond, ActiveTable, CondTable, sensor_act_values);
                }
            }

            Condition::Not { cond, watch_cond_id } => {
                if !CondTable[cond.cond_id]{
                    setVarSpawn(!CondTable[*watch_cond_id], cond, ActiveTable, CondTable, sensor_act_values);
                }
            }

            Condition::StartImmediately { cond } => {
                if !CondTable[cond.cond_id]{
                    setVarSpawn(true, cond, ActiveTable, CondTable, sensor_act_values);
                }
            }

            _ => {}
        }
    }
}


pub fn TerminateEvents(term_list: &Vec<Condition>, ActiveTable: &mut Vec<bool>, TerminatedTable: &mut Vec<bool>, CondTable: &mut Vec<bool>, _sensor_act_values: &mut SensorActuatorValues) {
    for _condition in term_list {
        match _condition {
            Condition::IsTerminated { cond, watch_process_id } => {
                if ActiveTable[cond.process_id] {
                    setVarTerm(TerminatedTable[*watch_process_id], cond, ActiveTable, TerminatedTable, CondTable);
                } 
            }

            Condition::And { cond, watch_cond_id0, watch_cond_id1 } => {
                if ActiveTable[cond.process_id] {
                    setVarTerm(CondTable[*watch_cond_id0] && CondTable[*watch_cond_id1], cond, ActiveTable, TerminatedTable, CondTable);
                }
            }

            Condition::Or { cond, watch_cond_id0, watch_cond_id1 } => {
                if ActiveTable[cond.process_id] {
                    setVarTerm(CondTable[*watch_cond_id0] || CondTable[*watch_cond_id1], cond, ActiveTable, TerminatedTable, CondTable);
                }
            }

            Condition::Not { cond, watch_cond_id } => {
                if ActiveTable[cond.process_id] {
                    setVarTerm(!CondTable[*watch_cond_id], cond, ActiveTable, TerminatedTable, CondTable);
                }
            }

            Condition::StopImmediately { cond } => {
                if  ActiveTable[cond.process_id] {
                    setVarTerm(true, cond, ActiveTable, TerminatedTable, CondTable);
                }
            }
            
            Condition::SensorValue { cond} => {
                if  ActiveTable[cond.process_id] {
                    setVarTerm(CondTable[cond.cond_id], cond, ActiveTable, TerminatedTable, CondTable);
                }
            }

            // Not relevant here
            _ => {}
        }
    }
}