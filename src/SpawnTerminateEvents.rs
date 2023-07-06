// Import crates
extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::{motors, Port};
use log::{error, info, warn};
use std::time::{Duration, Instant};

// Local modules
use crate::Events::{Condition, Event, CondID};
use crate::Ports::{MotorsSensors, PortDefinition};
use crate::ProcessLoop::SensorActuatorValues;


fn setVarSpawn(result: bool, cond: &CondID, ActiveTable: &mut Vec<bool>,CondTable: &mut Vec<bool>){
    if result {
        CondTable[cond.cond_id] = true;
        if cond.process_id != 0 {
            ActiveTable[cond.process_id] = true;
        }
    } else {
        CondTable[cond.cond_id] = false;
    }

}

fn setVarTerm(result: bool, cond: &CondID, ActiveTable: &mut Vec<bool>, TerminatedTable: &mut Vec<bool>, CondTable: &mut Vec<bool>){
    if result {
        CondTable[cond.cond_id] = true;
        if cond.process_id != 0 {
            ActiveTable[cond.process_id] = false;
            TerminatedTable[cond.process_id] = true;
        }
    } else {
        CondTable[cond.cond_id] = false;
    }

}

pub fn SpawnEvents(event_list: &mut Vec<Event>, spawn_list: &Vec<Condition>, ActiveTable: &mut Vec<bool>, TerminatedTable: &Vec<bool>, CondTable: &mut Vec<bool>) {
    for _condition in spawn_list {
        match _condition {
            Condition::None => {}

            Condition::IsTerminated { cond, watch_process_id } => {
                if !CondTable[cond.cond_id] {
                    setVarSpawn(TerminatedTable[*watch_process_id], cond, ActiveTable, CondTable);
                } 
            }

            Condition::And { cond, watch_cond_id0, watch_cond_id1 } => {
                if !CondTable[cond.cond_id]{
                    setVarSpawn(CondTable[*watch_cond_id0] && CondTable[*watch_cond_id1], cond, ActiveTable, CondTable)
                }
            }

            Condition::Or { cond, watch_cond_id0, watch_cond_id1 } => {
                if !CondTable[cond.cond_id]{
                    setVarSpawn(CondTable[*watch_cond_id0] || CondTable[*watch_cond_id1], cond, ActiveTable, CondTable)
                }
            }

            Condition::Not { cond, watch_cond_id } => {
                if !CondTable[cond.cond_id]{
                    setVarSpawn(!CondTable[*watch_cond_id], cond, ActiveTable, CondTable)
                }
            }

            Condition::StartImmediately { cond } => {
                if !CondTable[cond.cond_id]{
                    CondTable[cond.cond_id] = true;
                    if cond.process_id != 0 {
                        ActiveTable[cond.process_id] = true;
                        info!("spawned {}", cond.process_id);
                    }
                }
            }

            // Not relevant here
            Condition::StopImmediately { cond } => {}
            Condition::SensorValue { cond } => {}
            Condition::Timer { cond } => {}
        }
    }
}


pub fn TerminateEvents(event_list: &mut Vec<Event>, term_list: &Vec<Condition>, ActiveTable: &mut Vec<bool>, TerminatedTable: &mut Vec<bool>, CondTable: &mut Vec<bool>, sensor_act_values: &mut SensorActuatorValues) {
    for _condition in term_list {
        match _condition {
            Condition::None => {}

            Condition::IsTerminated { cond, watch_process_id } => {
                if ActiveTable[cond.process_id] {
                    setVarTerm(TerminatedTable[*watch_process_id], cond, ActiveTable, TerminatedTable, CondTable)
                } 
            }

            Condition::And { cond, watch_cond_id0, watch_cond_id1 } => {
                if ActiveTable[cond.process_id] {
                    setVarTerm(CondTable[*watch_cond_id0] && CondTable[*watch_cond_id1], cond, ActiveTable, TerminatedTable, CondTable)
                }
            }

            Condition::Or { cond, watch_cond_id0, watch_cond_id1 } => {
                if ActiveTable[cond.process_id] {
                    setVarTerm(CondTable[*watch_cond_id0] || CondTable[*watch_cond_id1], cond, ActiveTable, TerminatedTable, CondTable)
                }
            }

            Condition::Not { cond, watch_cond_id } => {
                if ActiveTable[cond.process_id] {
                    setVarTerm(!CondTable[*watch_cond_id], cond, ActiveTable, TerminatedTable, CondTable)
                }
            }

            Condition::StopImmediately { cond } => {
                if  ActiveTable[cond.process_id] {
                    CondTable[cond.cond_id] = true;
                    if cond.process_id != 0 {
                        ActiveTable[cond.process_id] = false;
                        TerminatedTable[cond.process_id] = true;
                    }
                }
            }
            
            Condition::SensorValue { cond} => {
                if  ActiveTable[cond.process_id] {
                    if cond.process_id != 0 && CondTable[cond.cond_id]{
                        ActiveTable[cond.process_id] = false;
                        TerminatedTable[cond.process_id] = true;
                    }
                }
            }

            Condition::Timer { cond} => {
                if  ActiveTable[cond.process_id] {
                    if cond.process_id != 0 && CondTable[cond.cond_id]{
                        ActiveTable[cond.process_id] = false;
                        TerminatedTable[cond.process_id] = true;
                    }
                }
            }

            // Not relevant here
            Condition::StartImmediately { cond } => {}
        }
    }
}