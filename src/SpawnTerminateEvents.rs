// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port};
use log::{error, info, warn};
use std::time::{Duration, Instant};

// Local modules
use super::DEBUG;
use crate::Events::{Condition, Event, CondID};
use crate::Ports::{MotorsSensors, PortDefinition};
use crate::ProcessLoop::SensorActuatorValues;



fn setVarSpawn(result: bool, cond: &CondID, ActiveTable: &mut Vec<bool>, CondTable: &mut Vec<bool>){
    CondTable[cond.cond_id] = result;
    if result {
        if cond.process_id != 0 {
            ActiveTable[cond.process_id] = true;
        }

        if DEBUG {
            info!("Condition ID {} spawned process ID {}.", cond.cond_id, cond.process_id);
        }
    }
}


fn setVarTerm(result: bool, cond: &CondID, ActiveTable: &mut Vec<bool>, TerminatedTable: &mut Vec<bool>, CondTable: &mut Vec<bool>){
    CondTable[cond.cond_id] = result;
    if result {
        if cond.process_id != 0 {
            ActiveTable[cond.process_id] = false;
            TerminatedTable[cond.process_id] = true;
            
            if DEBUG {
                info!("Condition ID {} terminated process ID {}.", cond.cond_id, cond.process_id);
            }
        }
    }
}


pub fn SpawnEvents(event_list: &mut Vec<Event>, spawn_list: &Vec<Condition>, ActiveTable: &mut Vec<bool>, TerminatedTable: &Vec<bool>, CondTable: &mut Vec<bool>) {
    for _condition in spawn_list {
        match _condition {
            Condition::IsTerminated { cond, watch_process_id } => {
                if !CondTable[cond.cond_id] {
                    setVarSpawn(TerminatedTable[*watch_process_id], cond, ActiveTable, CondTable);
                } 
            }

            Condition::And { cond, watch_cond_id0, watch_cond_id1 } => {
                if !CondTable[cond.cond_id]{
                    setVarSpawn(CondTable[*watch_cond_id0] && CondTable[*watch_cond_id1], cond, ActiveTable, CondTable);
                }
            }

            Condition::Or { cond, watch_cond_id0, watch_cond_id1 } => {
                if !CondTable[cond.cond_id]{
                    setVarSpawn(CondTable[*watch_cond_id0] || CondTable[*watch_cond_id1], cond, ActiveTable, CondTable);
                }
            }

            Condition::Not { cond, watch_cond_id } => {
                if !CondTable[cond.cond_id]{
                    setVarSpawn(!CondTable[*watch_cond_id], cond, ActiveTable, CondTable);
                }
            }

            Condition::StartImmediately { cond } => {
                if !CondTable[cond.cond_id]{
                    setVarSpawn(true, cond, ActiveTable, CondTable);
                }
            }

            _ => {}
        }
    }
}


pub fn TerminateEvents(event_list: &mut Vec<Event>, term_list: &Vec<Condition>, ActiveTable: &mut Vec<bool>, TerminatedTable: &mut Vec<bool>, CondTable: &mut Vec<bool>, sensor_act_values: &mut SensorActuatorValues) {
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