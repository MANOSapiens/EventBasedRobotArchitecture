use crate::Events::CondID;
use crate::Events::PID;
use crate::Events::{Condition, Event, EventID, FuncTypes};
use log::{error, info};
use serde_json::{value::Value};
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use rand::Rng;


fn parsei8(value: Option<&Value>) -> i8 {
    // debug!("{}", value.expect("Trying to read a non-number value into a number"));
    value
        .expect("Trying to read a non-number value into a number")
        .as_i64()
        .expect("parsei8: Cannot convert value into i64")
        .try_into()
        .expect("Not able to do type casting to i8")
}

fn parseusize(value: Option<&Value>) -> usize {
    value
        .expect("Trying to read a non-number value into a number")
        .as_i64()
        .expect("parseusize: Cannot convert value into i64")
        .try_into()
        .expect("Not able to do type casting to usize")
}

fn parsef32(value: Option<&Value>) -> f32 {
    let result: f32 = value
        .expect("Trying to read a non-number value into a number")
        .as_f64()
        .expect("parsef32: Cannot convert value into f64") as f32;
    result
}

fn parseString(value: Option<&Value>) -> &str {
    value
        .expect("Trying to read a non-string value into a string")
        .as_str()
        .expect("parsef32: Cannot convert value into f64")
}

fn readEvent(event: &serde_json::Value) -> EventID {
    let event_struct: &serde_json::Value = event
        .get("event")
        .expect("Event does not contain an Event metadata!");
    EventID {
        process_id: parseusize(event_struct.get("process_id")),
        spawn_conditions_id: parseusize(event_struct.get("spawn_conditions_id")),
        term_conditions_id: parseusize(event_struct.get("term_conditions_id")),
    }
}

fn readCond(cond: &serde_json::Value) -> CondID {
    let cond_struct: &serde_json::Value = cond
        .get("cond")
        .expect("Condition does not contain an Cond metadata!");
    CondID {
        process_id: parseusize(cond_struct.get("process_id")),
        cond_id: parseusize(cond_struct.get("cond_id")),
        sensor_needed: parsei8(cond_struct.get("sensor_needed"))
    }
}

fn readFunc(event: &serde_json::Value) -> FuncTypes {
    // debug!("reading function");
    let func: &serde_json::Value = event
        .get("func")
        .expect("Cannot parse func from MotorSpeedControl!");
    let func_type: &str = parseString(func.get("type"));
    match func_type {
        "constant" => {
            return FuncTypes::ConstFunc {
                c: parsef32(func.get("c")),
            }
        }
        "linear" => {
            return FuncTypes::LinearFunc {
                m: parsef32(func.get("m")),
                e: parsef32(func.get("e")),
                lb: parsef32(func.get("lb")),
                hb: parsef32(func.get("hb")),
                step_prev: -1.0,
            }
        }
        "quadratic" => {
            return FuncTypes::QuadFunc {
                a: parsef32(func.get("a")),
                b: parsef32(func.get("b")),
                c: parsef32(func.get("c")),
                lb: parsef32(func.get("lb")),
                hb: parsef32(func.get("hb")),
                step_prev: -1.0,
            }
        }
        _ => {
            error!("Function type {} unknown!", func_type);
            process::exit(0);
        }
    }
}

fn readPID(event: &serde_json::Value) -> PID {
    // debug!("reading pid");
    let pid: &serde_json::Value = event.get("pid").expect("Event does not contain PID!");
    PID {
        p: parsef32(pid.get("p")),
        i: parsef32(pid.get("i")),
        d: parsef32(pid.get("d")),
        max_i: parsef32(pid.get("max_i")),
        sum_i: 0.0,
        prev_e: -1.0,
    }
}

fn matchEvent(event: &serde_json::Value, event_list: &mut Vec<Event>) {
    // debug!("starting to parse events");
    let event_type: &str = parseString(event.get("type"));
    match event_type {
        "Placeholder" => event_list.push(Event::Placeholder {
            event: readEvent(event),
        }),

        "SensorValue" => event_list.push(Event::SensorValue {
            event: readEvent(event),
            sensor_target: parsef32(event.get("sensor_target")),
            sensor_prev: parsef32(event.get("sensor_prev")),
            sensor_id: parsei8(event.get("sensor_id")),
            expr: parseString(event.get("expr"))
                .chars().next()
                .expect("Cannot parse char from SensorValue!"),
            sensvalcondid: parseusize(event.get("sensvalcondid")),
        }),

        "MotorSpeedControl" => event_list.push(Event::MotorSpeedControl {
            event: readEvent(event),
            motor_id: parsei8(event.get("motor_id")),
            sensor_id: parsei8(event.get("sensor_id")),
            func: readFunc(event),
        }),

        "PIDGyro" => event_list.push(Event::PIDGyro {
            event: readEvent(event),
            heading: parsef32(event.get("heading")),
            pid: readPID(event),
            motor_correction: 0.0,
            sensor_prev: parsef32(event.get("sensor_prev")),
        }),

        "PIDLine" => event_list.push(Event::PIDLine {
            event: readEvent(event),
            brightness_target: parsef32(event.get("brightness_target")),
            pid: readPID(event),
            motor_correction: 0.0,
        }),

        "PIDHold" => event_list.push(Event::PIDHold {
            event: readEvent(event),
            pid: readPID(event),
            motor_correction: 0.0,
        }),

        "Timer" => event_list.push(Event::Timer {
            event: readEvent(event),
            time: parsef32(event.get("time")),
            time_prev: -1.0,
        }),

        "ComputeMotorStall" => event_list.push(Event::ComputeMotorStall {
            event: readEvent(event),
            min_mov_avg_speed: parsef32(event.get("min_mov_avg_speed")),
            buffer: VecDeque::with_capacity(parseusize(event.get("buffer_size"))),
            buffer_size: parseusize(event.get("buffer_size")),
            motor_id: parsei8(event.get("motor_id")),
        }),

        "HaltProcessLoop" => event_list.push(Event::HaltProcessLoop {
            event: readEvent(event),
        }),

        "None" => {}

        _ => {
            error!("Event type {} unknown!", event_type);
        }
    }
}

fn matchCond(condition: &serde_json::Value, list: &mut Vec<Condition>) {
    let condition_type: &str = parseString(condition.get("type"));
    // debug!("starting to parse conditions");
    match condition_type {
        "IsTerminated" => list.push(Condition::IsTerminated {
            cond: readCond(condition),
            watch_process_id: parseusize(condition.get("watch_process_id")),
        }),

        "And" => list.push(Condition::And {
            cond: readCond(condition),
            watch_cond_id0: parseusize(condition.get("watch_cond_id0")),
            watch_cond_id1: parseusize(condition.get("watch_cond_id1")),
        }),

        "Or" => list.push(Condition::Or {
            cond: readCond(condition),
            watch_cond_id0: parseusize(condition.get("watch_cond_id0")),
            watch_cond_id1: parseusize(condition.get("watch_cond_id1")),
        }),

        "Not" => list.push(Condition::Not {
            cond: readCond(condition),
            watch_cond_id: parseusize(condition.get("watch_cond_id")),
        }),

        "StartImmediately" => list.push(Condition::StartImmediately {
            cond: readCond(condition),
        }),

        "StopImmediately" => list.push(Condition::StopImmediately {
            cond: readCond(condition),
        }),

        "SensorValue" => list.push(Condition::SensorValue {
            cond: readCond(condition),
        }),

        "Placeholder" => list.push(Condition::Placeholder {
            cond: readCond(condition),
        }),

        _ => {
            error!(
                "Condition type {} unknown!",
                condition_type
            );
        }
    }
}

pub fn ReadInstructions(
    file_path: &PathBuf,
    spawn_list: &mut Vec<Condition>,
    event_list: &mut Vec<Event>,
    term_list: &mut Vec<Condition>,
    round_timeout: &mut f32,
    name: &mut String,
) {
    let file = fs::File::open(file_path).expect("Instructions file is not available!");
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();

    let event_list_len: usize = parseusize(json.get("EventListLen"));
    let spawn_list_len: usize = parseusize(json.get("SpawnListLen"));
    let term_list_len: usize = parseusize(json.get("TermListLen"));

    let time_now: Instant = Instant::now();
    *name = generateName(parseString(json.get("name")));
    info!("================== {} ====================", name);
    info!("Reading JSON file {}", file_path.display());
    *round_timeout = parsef32(json.get("round_timeout"));


    for i in 0..event_list_len {
        let event: &serde_json::Value = json["EventList"]
            .get(i as usize)
            .expect("Event at index not available");

        matchEvent(event, event_list);
    }

    for i in 0..spawn_list_len {
        let condition: &serde_json::Value = json["SpawnList"]
            .get(i as usize)
            .expect("Spawn condition at index not available");
        matchCond(condition, spawn_list);
    }

    for i in 0..term_list_len {
        let condition: &serde_json::Value = json["TermList"]
            .get(i as usize)
            .expect("Term condition at index not available");
        matchCond(condition, term_list);
    }

    info!("Finished reading JSON. This took {}s", time_now.elapsed().as_secs_f32());
}


pub fn generateName(round_name: &str) -> String {
    // Common adjectives
    let adjectives: [&str; 48] = [
        "Happy", "Sad", "Angry", "Beautiful", "Ugly", "Tall", "Short", "Long", "Thin", "Fat",
        "Smart", "Dumb", "Brave", "Cowardly", "Fast", "Slow", "Bright", "Dark", "Loud", "Quiet",
        "Clever", "Clumsy", "Friendly", "Hostile", "Generous", "Stingy", "Kind", "Cruel",
        "Polite", "Rude", "Honest", "Dishonest", "Strong", "Weak", "Healthy", "Sick", "Clean",
        "Dirty", "Famous", "Unknown", "Noisy", "Silent", "Pleasant", "Unpleasant", "Modern",
        "Ancient", "Rich", "Poor",
    ];

    // Common objects
    let objects: [&str; 25] = [
        "Chair",
        "Table",
        "Computer",
        "Phone",
        "Car",
        "Book",
        "Pen",
        "Pencil",
        "Coffee mug",
        "Television",
        "Refrigerator",
        "Clock",
        "Wallet",
        "Keys",
        "Sunglasses",
        "Shoes",
        "Bicycle",
        "Watch",
        "Backpack",
        "Hat",
        "Spoon",
        "Fork",
        "Knife",
        "Plate",
        "Remote control",
    ];

    let mut rng = rand::thread_rng();
    let random_obj = rng.gen_range(0..objects.len());
    let random_adj0 = rng.gen_range(0..adjectives.len());

    let obj: &str = objects[random_obj];
    let adj0: &str = adjectives[random_adj0];

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    format!("{round_name}|{adj0}-{obj}|{:?}", since_the_epoch.as_secs())
}