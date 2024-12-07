from spike_prime import  consts
from spike_prime.Events import *
import json, time

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
    return f'A({parse_int(e["process_id"])}, {parse_int(e["spawn_conditions_id"])}, {parse_int(e["term_conditions_id"])})'
    return EventID(
        process_id=parse_int(e["process_id"]),
        spawn_conditions_id=parse_int(e["spawn_conditions_id"]),
        term_conditions_id=parse_int(e["term_conditions_id"]),
    )

def read_cond(cond):
    c = cond["cond"]
    return f'B({parse_int(c["process_id"])}, {parse_int(c["cond_id"])}, {parse_int(c["sensor_needed"])})'
    return CondID(
        process_id=parse_int(c["process_id"]),
        cond_id=parse_int(c["cond_id"]),
        sensor_needed=parse_int(c["sensor_needed"])
    )

def read_func(func):
    func_type = parse_string(func['type'])
    if func_type == "constant":
        return f'D({parse_f32(func["c"])})'
        return ConstFunc(c=parse_f32(func["c"]))
    elif func_type == "linear":
        return f'E({parse_f32(func["m"])}, {parse_f32(func["e"])}, {parse_f32(func["lb"])}, {parse_f32(func["hb"])})'
        return LinearFunc(
            m=parse_f32(func["m"]),
            e=parse_f32(func["e"]),
            lb=parse_f32(func["lb"]),
            hb=parse_f32(func["hb"]),
            step_prev=-1.0,
        )
    else:
        raise ValueError("Function type {} unknown!".format(func_type))

def read_pid(pid_data):
    return f'C({parse_f32(pid_data["p"])}, {parse_f32(pid_data["i"])}, {parse_f32(pid_data["d"])}, {parse_f32(pid_data["max_i"])})'
    return PID(
        p=parse_f32(pid_data["p"]),
        i=parse_f32(pid_data["i"]),
        d=parse_f32(pid_data["d"]),
        max_i=parse_f32(pid_data["max_i"]),
        sum_i=0.0,
        prev_e=-1.0,
    )

def match_event(event):
    event_type = parse_string(event.get("type"))
    if event_type == "Placeholder":
        return f'F({read_event(event)})'
        return Placeholder(event=read_event(event))

    elif event_type == "SensorValue":
        return f'G({read_event(event)}, {parse_f32(event["sensor_target"])}, {parse_f32(event["sensor_prev"])}, {parse_int(event["sensor_id"])}, {SMALLERTHAN_ if parse_string(event["expr"]) == "<" else LAGERTHAN_}, {parse_int(event["sensvalcondid"])})'
        return SensorValue(
                event=read_event(event),
                sensor_target=parse_f32(event["sensor_target"]),
                sensor_prev=parse_f32(event["sensor_prev"]),
                sensor_id=parse_int(event["sensor_id"]),
                expr=SMALLERTHAN_ if parse_string(event["expr"]) == '<' else LAGERTHAN_,
                sensvalcondid=parse_int(event["sensvalcondid"]),
            )

    elif event_type == "MotorSpeedControl":
        return f'H({read_event(event)}, {parse_int(event["motor_id"])}, {parse_int(event["sensor_id"])}, {read_func(event["func"])})'
        return MotorSpeedControl(
                event=read_event(event),
                motor_id=parse_int(event["motor_id"]),
                sensor_id=parse_int(event["sensor_id"]),
                func=read_func(event['func']),
            )


    elif event_type == "PIDGyro":
        return f'I({read_event(event)}, {parse_f32(event["heading"])}, {read_pid(event["pid"])}, {parse_f32(event["sensor_prev"])})'
        return PIDGyro(
                event=read_event(event),
                heading=parse_f32(event["heading"]),
                pid=read_pid(event['pid']),
                sensor_prev=parse_f32(event["sensor_prev"]),
            )


    elif event_type == "Timer":
        return f'J({read_event(event)}, {parse_f32(event["time"])}, -1.0)'
        return Timer(
                event=read_event(event),
                time=parse_f32(event["time"]),
                time_prev=-1.0,
            )


    elif event_type == "ComputeMotorStall":
        return f'K({read_event(event)}, {parse_f32(event["min_mov_avg_speed"])}, {parse_int(event["buffer_size"])}, {parse_int(event["motor_id"])})'
        return ComputeMotorStall(
                event=read_event(event),
                min_mov_avg_speed=parse_f32(event["min_mov_avg_speed"]),
                buffer_size=parse_int(event["buffer_size"]),
                motor_id=parse_int(event["motor_id"]),
            )


    elif event_type == "HaltProcessLoop":
        return f'L({read_event(event)})'
        return HaltProcessLoop(
                event=read_event(event),
            )


    else:
        print("Unknown event type {}".format(event_type))



def match_cond(condition):
    condition_type = parse_string(condition.get("type"))
    if condition_type == "IsTerminated":
        return f'N({read_cond(condition)}, {parse_int(condition["watch_process_id"])})'
        return IsTerminated(
            cond=read_cond(condition),
            watch_process_id=parse_int(condition["watch_process_id"]),
        )
    elif condition_type == "And":
        return f'O({read_cond(condition)}, {parse_int(condition["watch_cond_id0"])}, {parse_int(condition["watch_cond_id1"])})'
        return And(
            cond=read_cond(condition),
            watch_cond_id0=parse_int(condition["watch_cond_id0"]),
            watch_cond_id1=parse_int(condition["watch_cond_id1"]),
        )

    elif condition_type == "Or":
        return f'P({read_cond(condition)}, {parse_int(condition["watch_cond_id0"])}, {parse_int(condition["watch_cond_id1"])})'
        return Or(
            cond=read_cond(condition),
            watch_cond_id0=parse_int(condition["watch_cond_id0"]),
            watch_cond_id1=parse_int(condition["watch_cond_id1"]),
        )

    elif condition_type == "Not":
        return f'Q({read_cond(condition)}, {parse_int(condition["watch_cond_id"])})'
        return Not(
            cond=read_cond(condition),
            watch_cond_id=parse_int(condition["watch_cond_id"]),
        )

    elif condition_type == "StartImmediately":
        return f'R({read_cond(condition)})'
        return StartImmediately(
            cond=read_cond(condition),
        )

    elif condition_type == "StopImmediately":
        return f'S({read_cond(condition)})'
        return StopImmediately(
            cond=read_cond(condition),
        )

    elif condition_type == "SensorValue":
        return f'T({read_cond(condition)})'
        return SensorValueCond(
            cond=read_cond(condition),
        )

    elif condition_type == "Placeholder":
        return f'M({read_cond(condition)})'
        return PlaceholderCond(
            cond=read_cond(condition),
        )

    else:
        raise print("Unknown condition type {}".format(condition_type))


def read_instructions(file_path):
    event_list = []
    spawn_list = []
    term_list = []

    with open(file_path, 'r') as file:
        # Parse the JSON file
        data = json.load(file)
    
    for event in data["EventList"]:
        event_list.append(match_event(event))
    
    for cond in data['SpawnList']:
        spawn_list.append(match_cond(cond))
    
    for cond in data['TermList']:
        term_list.append(match_cond(cond))
    
    metadata = [data["name"], str(parse_f32(data["round_timeout"])), str(data["file_forward"])]
    
    return metadata, event_list, spawn_list, term_list

def make_printable_list(list):
    return '[' + ', '.join(list) + ']'

metadata, event_list, spawn_list, term_list = read_instructions('D:/ev3EventArchitecture/instructions/Runde1.json')
print(make_printable_list(event_list))
print(make_printable_list(spawn_list))
print(make_printable_list(term_list))
print(make_printable_list(metadata))