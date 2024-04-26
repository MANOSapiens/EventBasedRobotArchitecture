from Events import *

import random
import time
import json


def parse_int(value):
    if value is None:
        raise ValueError("Trying to read a non-number value into a number")
    return int(value)

def parse_f32(value):
    if value is None:
        raise ValueError("Trying to read a non-number value into a number")
    return float(value)

def parse_string(value):
    if value is None or not isinstance(value, str):
        raise ValueError("Trying to read a non-string value into a string")
    return value


def read_event(event):
    e = event["event"]
    return EventID(
        process_id=parse_int(e["process_id"]),
        spawn_conditions_id=parse_int(e["spawn_conditions_id"]),
        term_conditions_id=parse_int(e["term_conditions_id"]),
    )

def read_cond(cond):
    c = cond["cond"]
    return CondID(
        process_id=parse_int(c["process_id"]),
        cond_id=parse_int(c["cond_id"]),
        sensor_needed=parse_int(c["sensor_needed"])
    )

def read_func(func):
    func_type = parse_string(func['type'])
    if func_type == "constant":
        return ConstFunc(c=parse_f32(func["c"]))
    elif func_type == "linear":
        return LinearFunc(
            m=parse_f32(func["m"]),
            e=parse_f32(func["e"]),
            lb=parse_f32(func["lb"]),
            hb=parse_f32(func["hb"]),
            step_prev=-1.0,
        )
    elif func_type == "quadratic":
        return QuadFunc(
            a=parse_f32(func["a"]),
            b=parse_f32(func["b"]),
            c=parse_f32(func["c"]),
            lb=parse_f32(func["lb"]),
            hb=parse_f32(func["hb"]),
            step_prev=-1.0,
        )
    else:
        raise ValueError(f"Function type {func_type} unknown!")

def read_pid(pid_data):
    
    return PID(
        p=parse_f32(pid_data["p"]),
        i=parse_f32(pid_data["i"]),
        d=parse_f32(pid_data["d"]),
        max_i=parse_f32(pid_data["max_i"]),
        sum_i=0.0,
        prev_e=-1.0,
    )

def match_event(event, notetaker):
    event_type = parse_string(event.get("type"))
    if event_type == "Placeholder":
        return Placeholder(event=read_event(event))

    elif event_type == "SensorValue":
        return SensorValue(
                event=read_event(event),
                sensor_target=parse_f32(event["sensor_target"]),
                sensor_prev=parse_f32(event["sensor_prev"]),
                sensor_id=parse_int(event["sensor_id"]),
                expr=parse_string(event["expr"]),
                sensvalcondid=parse_int(event["sensvalcondid"]),
            )
        
    elif event_type == "MotorSpeedControl":
        return MotorSpeedControl(
                event=read_event(event),
                motor_id=parse_int(event["motor_id"]),
                sensor_id=parse_int(event["sensor_id"]),
                func=read_func(event['func']),
            )
        

    elif event_type == "PIDGyro":
        return PIDGyro(
                event=read_event(event),
                heading=parse_f32(event["heading"]),
                pid=read_pid(event['pid']),
                motor_correction=0.0,
                sensor_prev=parse_f32(event["sensor_prev"]),
            )
        

    elif event_type == "PIDLine":
        return PIDLine(
                event=read_event(event),
                brightness_target=parse_f32(event["brightness_target"]),
                pid=read_pid(event['pid']),
                motor_correction=0.0,
            )
        

    elif event_type == "PIDHold":
        return PIDHold(
                event=read_event(event),
                pid=read_pid(event['pid']),
                motor_correction=0.0,
            )
        

    elif event_type == "Timer":
        return Timer(
                event=read_event(event),
                time=parse_f32(event["time"]),
                time_prev=-1.0,
            )
        

    elif event_type == "ComputeMotorStall":
        return ComputeMotorStall(
                event=read_event(event),
                min_mov_avg_speed=parse_f32(event["min_mov_avg_speed"]),
                buffer_size=parse_int(event["buffer_size"]),
                motor_id=parse_int(event["motor_id"]),
            )
        
        
    elif event_type == "HaltProcessLoop":
        return HaltProcessLoop(
                event=read_event(event),
            )
        
        
    else:
        notetaker.error(f"Unknown event type {event_type}")



def match_cond(condition, notetaker):
    condition_type = parse_string(condition.get("type"))
    if condition_type == "IsTerminated":
        return IsTerminated(
            cond=read_cond(condition),
            watch_process_id=parse_int(condition["watch_process_id"]),
        )
    elif condition_type == "And":
        return And(
            cond=read_cond(condition),
            watch_cond_id0=parse_int(condition["watch_cond_id0"]),
            watch_cond_id1=parse_int(condition["watch_cond_id1"]),
        )
        
    elif condition_type == "Or":
        return Or(
            cond=read_cond(condition),
            watch_cond_id0=parse_int(condition["watch_cond_id0"]),
            watch_cond_id1=parse_int(condition["watch_cond_id1"]),
        )
    
    elif condition_type == "Not":
        return Not(
            cond=read_cond(condition),
            watch_cond_id=parse_int(condition["watch_cond_id"]),
        )
        
    elif condition_type == "StartImmediately":
        return StartImmediately(
            cond=read_cond(condition),
        )
        
    elif condition_type == "StopImmediately":
        return StopImmediately(
            cond=read_cond(condition),
        )
        
    elif condition_type == "SensorValue":
        return SensorValueCond(
            cond=read_cond(condition),
        )
        
    elif condition_type == "Placeholder":
        return PlaceholderCond(
            cond=read_cond(condition),
        )
        
    else:
        raise notetaker.error(f"Unknown condition type {condition_type}")





def read_instructions(file_path, notetaker):
    event_list = []
    spawn_list = []
    term_list = []
    time_now = time.time()

    with open(file_path, 'r') as file:
        # Parse the JSON file
        data = json.load(file)

    name = generate_name(data["name"])

    notetaker.log(f'================== {name} ====================')
    notetaker.log('Reading JSON file {}'.format(file_path))

    for event in data["EventList"]:
        event_list.append(match_event(event, notetaker))

    for cond in data['SpawnList']:
        spawn_list.append(match_cond(cond, notetaker))

    for cond in data['TermList']:
        term_list.append(match_cond(cond, notetaker))

    notetaker.log(f"Finished reading JSON. This took {time.time()-time_now}s")

    return name, parse_f32(data["round_timeout"]), parse_f32(data["speed_p"]), parse_f32(data["speed_i"]), parse_f32(data["speed_d"]), data["file_forward"], event_list, spawn_list, term_list

def generate_name(round_name):
    adjectives = ["Happy", "Sad", "Angry", "Beautiful", "Ugly", "Tall", "Short", "Long", "Thin", "Fat",
        "Smart", "Dumb", "Brave", "Cowardly", "Fast", "Slow", "Bright", "Dark", "Loud", "Quiet",
        "Clever", "Clumsy", "Friendly", "Hostile", "Generous", "Stingy", "Kind", "Cruel",
        "Polite", "Rude", "Honest", "Dishonest", "Strong", "Weak", "Healthy", "Sick", "Clean",
        "Dirty", "Famous", "Unknown", "Noisy", "Silent", "Pleasant", "Unpleasant", "Modern",
        "Ancient", "Rich", "Poor",]
    objects = ["Chair",
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
        "Remote control",]
    random_adj = random.choice(adjectives)
    random_obj = random.choice(objects)
    current_timestamp = int(time.time())
    return f"{current_timestamp}-{round_name}-{random_adj}-{random_obj}"