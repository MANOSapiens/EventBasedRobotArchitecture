# ==================== imports.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 

# Librarys
import array
from math import sin, radians
import random
import time
import ujson
import time
import os

import motor
import color
from hub import port, motion_sensor, button, light_matrix, light

# ==================== consts.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 

# Adjustable parameters
DEBUG_ = False
SPARSE_SENSOR_READING_ = True

# Fixed parameters for sensor and motor identifiers
LDRIVEENC = 0
RDRIVEENC = 1
LTOOLENC = 2
RTOOLENC = 3
COLOURSENS = 4
GYRO = 5
LDRIVEPOW = 6
RDRIVEPOW = 7
LTOOLPOW = 8
RTOOLPOW = 9
LDRIVECOR = 10
RDRIVECOR = 11
LTOOLCOR = 12
RTOOLCOR = 13
LDRIVEPOWPREV = 14
RDRIVEPOWPREV = 15
LTOOLPOWPREV = 16
RTOOLPOWPREV = 17
RIGHTBUTTON = 18
DRIVEENC = 19
DRIVESPEED = 20
LDRIVESPEED = 21
RDRIVESPEED = 22
LTOOLSPEED = 23
RTOOLSPEED = 24
TIME = 25
PREVTIME = 26
LDRIVEENCREAD = 27
RDRIVEENCREAD = 28
LTOOLENCREAD = 29
RTOOLENCREAD = 30
LDRIVESPEEDREAD = 31
RDRIVESPEEDREAD = 32
LTOOLSPEEDREAD = 33
RTOOLSPEEDREAD = 34
GYROREAD = 35
RIGHTBUTTONREAD = 36
DRIVEMOTORENCPREV = 37
GYROPREV = 38
GYROOFFSET = 39




# Event types
EVENT_NONE = 0
EVENT_PLACEHOLDER = 1
EVENT_SENSORVALUE = 2
EVENT_MOTORSPEEDCONTROL = 3
EVENT_PIDGYRO = 4
EVENT_PIDLINE = 5
EVENT_PIDHOLD = 6
EVENT_TIMER = 7
EVENT_COMPUTEMOTORSTALL = 8
EVENT_HALT = 9

FUNC_CONST = 9
FUNC_LINEAR = 10
FUNC_QUADRATIC = 11

CONDITION_NONE = 12
CONDITION_PLACEHOLDER = 13
CONDITION_ISTERMINATED = 14
CONDITION_SENSORVALUE = 15
CONDITION_AND = 16
CONDITION_OR = 17
CONDITION_NOT = 18
CONDITIONS_STARTIMMEDIATELY = 19
CONDITION_STOPIMMEDIATELY = 20

# File path
TABLELENGTHSFILE = "/flash/instructions/tableLengths"

# ==================== Events.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 



class EventID:
    def __init__(self, process_id: int, spawn_conditions_id: int, term_conditions_id: int):
        self.process_id = process_id
        self.spawn_conditions_id = spawn_conditions_id
        self.term_conditions_id = term_conditions_id

class CondID:
    def __init__(self, process_id: int, cond_id: int, sensor_needed: int):
        self.process_id = process_id
        self.cond_id = cond_id
        self.sensor_needed = sensor_needed

class PID:
    def __init__(self, p: float, i: float, d: float, max_i: float, sum_i: float, prev_e: float):
        self.p = p
        self.i = i
        self.d = d
        self.max_i = max_i
        self.sum_i = sum_i
        self.prev_e = prev_e


class ConstFunc:
    def __init__(self, c: float):
        self.c = c
        self.t = FUNC_CONST

class LinearFunc:
    def __init__(self, m: float, e: float, lb: float, hb: float, step_prev: float):
        self.m = m
        self.e = e
        self.lb = lb
        self.hb = hb
        self.step_prev = step_prev
        self.t = FUNC_LINEAR

class QuadFunc:
    def __init__(self, a: float, b: float, c: float, lb: float, hb: float, step_prev: float):
        self.a = a
        self.b = b
        self.c = c
        self.lb = lb
        self.hb = hb
        self.step_prev = step_prev
        self.t = FUNC_QUADRATIC

class Placeholder:
    def __init__(self, event: EventID):
        self.event = event

class SensorValue:
    def __init__(self, event: EventID, sensor_target: float, sensor_prev: float, sensor_id: int, expr: str, sensvalcondid: int):
        self.event = event
        self.sensor_target = sensor_target
        self.sensor_prev = sensor_prev
        self.sensor_id = sensor_id
        self.expr = expr
        self.sensvalcondid = sensvalcondid
        self.t = EVENT_SENSORVALUE

class MotorSpeedControl:
    def __init__(self, event: EventID, motor_id: int, sensor_id: int, func):
        self.event = event
        self.motor_id = motor_id
        self.sensor_id = sensor_id
        self.func = func
        self.t = EVENT_MOTORSPEEDCONTROL

class PIDGyro:
    def __init__(self, event: EventID, heading: float, pid: PID, motor_correction: float, sensor_prev: float):
        self.event = event
        self.heading = heading
        self.pid = pid
        self.motor_correction = motor_correction
        self.sensor_prev = sensor_prev
        self.t = EVENT_PIDGYRO

class PIDLine:
    def __init__(self, event: EventID, brightness_target: float, pid: PID, motor_correction: float):
        self.event = event
        self.brightness_target = brightness_target
        self.pid = pid
        self.motor_correction = motor_correction
        self.t = EVENT_PIDLINE

class PIDHold:
    def __init__(self, event: EventID, pid: PID, motor_correction: float):
        self.event = event
        self.pid = pid
        self.motor_correction = motor_correction
        self.t = EVENT_PIDHOLD

class Timer:
    def __init__(self, event: EventID, time: float, time_prev: float):
        self.event = event
        self.time = time
        self.time_prev = time_prev
        self.t = EVENT_TIMER

class ComputeMotorStall:
    def __init__(self, event: EventID, min_mov_avg_speed: float, buffer_size: int, motor_id: int):
        self.event = event
        self.min_mov_avg_speed = min_mov_avg_speed
        self.buffer = array.array('f', [0.0] * buffer_size)
        self.buffer_size = buffer_size
        self.motor_id = motor_id
        self.t = EVENT_COMPUTEMOTORSTALL

class HaltProcessLoop:
    def __init__(self, event: EventID):
        self.event = event
        self.t = EVENT_HALT


class PlaceholderCond:
    def __init__(self, cond: CondID):
        self.cond = cond
        self.t = CONDITION_PLACEHOLDER

    def __call__(self, cond_table, terminated_table):
        return False

class IsTerminated:
    def __init__(self, cond: CondID, watch_process_id: int):
        self.cond = cond
        self.watch_process_id = watch_process_id
        self.t = CONDITION_ISTERMINATED

    def __call__(self, cond_table, terminated_table):
        return terminated_table[self.watch_process_id]

class And:
    def __init__(self, cond: CondID, watch_cond_id0: int, watch_cond_id1: int):
        self.cond = cond
        self.watch_cond_id0 = watch_cond_id0
        self.watch_cond_id1 = watch_cond_id1
        self.t = CONDITION_AND

    def __call__(self, cond_table, terminated_table):
        return cond_table[self.watch_cond_id0] and cond_table[self.watch_cond_id1]

class Or:
    def __init__(self, cond: CondID, watch_cond_id0: int, watch_cond_id1: int):
        self.cond = cond
        self.watch_cond_id0 = watch_cond_id0
        self.watch_cond_id1 = watch_cond_id1
        self.t = CONDITION_OR

    def __call__(self, cond_table, terminated_table):
        return cond_table[self.watch_cond_id0] or cond_table[self.watch_cond_id1]

class Not:
    def __init__(self, cond: CondID, watch_cond_id: int):
        self.cond = cond
        self.watch_cond_id = watch_cond_id
        self.t = CONDITION_NOT

    def __call__(self, cond_table, terminated_table):
        return not cond_table[self.watch_cond_id]

class StartImmediately:
    def __init__(self, cond: CondID):
        self.cond = cond
        self.t = CONDITIONS_STARTIMMEDIATELY

    def __call__(self, cond_table, terminated_table):
        return True

class StopImmediately:
    def __init__(self, cond: CondID):
        self.cond = cond
        self.t = CONDITION_STOPIMMEDIATELY

    def __call__(self, cond_table, terminated_table):
        return True

class SensorValueCond:
    def __init__(self, cond: CondID):
        self.cond = cond
        self.t = CONDITION_SENSORVALUE

    def __call__(self, cond_table, terminated_table):
        return cond_table[self.cond.cond_id]

# ==================== ReadInstructions.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 



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
        raise ValueError("Function type {} unknown!".format(func_type))

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
        notetaker.error("Unknown event type {}".format(event_type))



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
        raise notetaker.error("Unknown condition type {}".format(condition_type))





def read_instructions(file_path, notetaker):
    event_list = []
    spawn_list = []
    term_list = []
    time_now = time.ticks_ms()/1000

    with open(file_path, 'r') as file:
        # Parse the JSON file
        data = ujson.load(file)

    name = generate_name(data["name"])

    notetaker.log('================== {} ===================='.format(name))
    notetaker.log('Reading JSON file {}'.format(file_path))

    for event in data["EventList"]:
        event_list.append(match_event(event, notetaker))
    del data["EventList"]

    for cond in data['SpawnList']:
        spawn_list.append(match_cond(cond, notetaker))
    del data["SpawnList"]

    for cond in data['TermList']:
        term_list.append(match_cond(cond, notetaker))
    del data["TermList"]

    notetaker.log("Finished reading JSON. This took {}s".format(time.ticks_ms()/1000-time_now))
    
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
    current_timestamp = int(time.ticks_ms()/1000)
    return "{}-{}-{}-{}".format(current_timestamp, round_name, random_adj, random_obj)



# ==================== Logger.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 


class Notetaker:
    def __init__(self, log_file):
        self.log_file = log_file
        self.file = open(log_file, 'a+')

    def log(self, message):
        print('[INFO] {}\n'.format(message))
        self.file.write('[INFO] {}\n'.format(message))
        self.file.flush()

    def warn(self, message):
        self.file.write('[WARNING] {}\n'.format(message))
        self.file.flush()

    def error(self, message):
        self.file.write('[ERROR] {}\n'.format(message))
        self.file.flush()


def init_logger(log_file):
    file = open(log_file, 'w')
    return file


def log_header_csv(writer):
    headers = [
        "currentTime",
        "lDriveMotorEnc",
        "rDriveMotorEnc",
        "lToolMotorEnc",
        "rToolMotorEnc",
        "gyroAngValue",
        "lDriveMotorPow",
        "rDriveMotorPow",
        "lToolMotorPow",
        "rToolMotorPow",
        "lDriveMotorCor",
        "rDriveMotorCor",
        "lDriveSpeed",
        "rDriveSpeed",
    ]
    writer.write(','.join(headers)+'\n')
    # Ensure the writer is flushed to write headers immediately
    writer.flush()


def log_csv(writer, sensor_act_values):
    row = [
        sensor_act_values[TIME],
        sensor_act_values[LDRIVEENC],
        sensor_act_values[RDRIVEENC],
        sensor_act_values[LTOOLENC],
        sensor_act_values[RTOOLENC],
        sensor_act_values[GYRO],
        sensor_act_values[LDRIVEPOW],
        sensor_act_values[RDRIVEPOW],
        sensor_act_values[LTOOLPOW],
        sensor_act_values[RTOOLPOW],
        sensor_act_values[LDRIVECOR],
        sensor_act_values[RDRIVECOR],
        sensor_act_values[LDRIVESPEED],
        sensor_act_values[RDRIVESPEED],
    ]
    writer.write(','.join([str(i) for i in row])+'\n')
    # Ensure the writer is flushed to save the data immediately
    writer.flush()

# ==================== Actuators.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 




def set_motor_pow(motor_pow, motor_id, sensor_act_values):
    if motor_id >= 6 and motor_id <= 17:
        sensor_act_values[motor_id] = motor_pow
    else:
        if DEBUG_:
            print("Motor ID {} unknown while assigning a power through setMotorPow()".format(motor_id))


def constrain_actuator_values(value):
    if value > 1.0:
        value = 1.0
    elif value < -1.0:
        value = -1.0
    return value


def write_to_actuators(motors_sensors, sensor_act_values):
    lDriveMotorPow = sensor_act_values[LDRIVEPOW] + sensor_act_values[LDRIVECOR]
    rDriveMotorPow = sensor_act_values[RDRIVEPOW] + sensor_act_values[RDRIVECOR]
    lToolMotorPow = sensor_act_values[LTOOLPOW] # + sensor_act_values[LTOOLCOR]
    rToolMotorPow = sensor_act_values[RTOOLPOW] # + sensor_act_values[RTOOLCOR]

    if lDriveMotorPow != sensor_act_values[LDRIVEPOWPREV]:
        lDriveMotorPow = constrain_actuator_values(lDriveMotorPow)
        if lDriveMotorPow == 0.0:
            motor.stop(motors_sensors.lDriveMotor)
        else:
            motor.run(motors_sensors.lDriveMotor, -int(lDriveMotorPow * 1000))
        sensor_act_values[LDRIVEPOWPREV] = lDriveMotorPow

    if rDriveMotorPow != sensor_act_values[RDRIVEPOWPREV]:
        rDriveMotorPow = constrain_actuator_values(rDriveMotorPow)
        if rDriveMotorPow == 0.0:
            motor.stop(motors_sensors.rDriveMotor)
        else:
            motor.run(motors_sensors.rDriveMotor, int(rDriveMotorPow * 1000))
        sensor_act_values[RDRIVEPOWPREV] = rDriveMotorPow

    if lToolMotorPow != sensor_act_values[LTOOLPOWPREV]:
        lToolMotorPow = constrain_actuator_values(lToolMotorPow)
        motor.run(motors_sensors.rDriveMotor, int(lToolMotorPow * 100))
        sensor_act_values[LTOOLPOWPREV] = lToolMotorPow

    if rToolMotorPow != sensor_act_values[RTOOLPOWPREV]:
        rToolMotorPow = constrain_actuator_values(rToolMotorPow)
        motor.run(motors_sensors.rToolMotor, int(rToolMotorPow * 100))
        sensor_act_values[RTOOLPOWPREV] = rToolMotorPow

# ==================== Check.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 



class RoundSummary:
    def __init__(self):
        self.wall_time = 0
        self.mean_loop_time = 0.0
        self.mean_f = 0 # Frequency in Hz
        self.loop_count = 0
        self.max_loop_time = 0.0
        self.total_travelled_distance = 0


def check(round_summary, sensor_act_values):
    round_summary.loop_count += 1
    if sensor_act_values[PREVTIME] == -1.0:
        sensor_act_values[PREVTIME] = sensor_act_values[TIME]

    elapsed = sensor_act_values[TIME] - sensor_act_values[PREVTIME]

    # Take down maximum loop time
    if round_summary.max_loop_time < elapsed:
        round_summary.max_loop_time = elapsed

# ==================== PID.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 


def compute_pid(value, target, delta_t, pid):
    error = target - value

    if pid.prev_e < 0.0:
        pid.prev_e = error

    pid.sum_i += pid.i * error * delta_t * 1000.0
    if pid.sum_i > pid.max_i:
        pid.sum_i = pid.max_i
    elif pid.sum_i < -pid.max_i:
        pid.sum_i = -pid.max_i

    if delta_t == 0.0:
        delta_t = 1000
    result = pid.p * error + (pid.d * (error - pid.prev_e)) / (delta_t * 1000.0) + pid.sum_i

    pid.prev_e = error
    return result

def compute_pid_gyro(value, target, delta_s, pid):
    error = target - value

    if pid.prev_e < 0.0:
        pid.prev_e = error


    pid.sum_i += pid.i * error * sin(radians(error)) * delta_s
    if pid.sum_i > pid.max_i:
        pid.sum_i = pid.max_i
    elif pid.sum_i < -pid.max_i:
        pid.sum_i = -pid.max_i

    result = pid.p * error + pid.d * (error - pid.prev_e) + pid.sum_i

    pid.prev_e = error
    return result


# ==================== Port.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 

class PortDefinition:
    def __init__(self, l_drive_port, r_drive_port, l_tool_port, r_tool_port, colour_port):
        self.lDriveMotorPort = l_drive_port
        self.rDriveMotorPort = r_drive_port
        self.lToolMotorPort = l_tool_port
        self.rToolMotorPort = r_tool_port
        self.colourSensPort = colour_port

class MotorsSensors:
    def __init__(self, lDriveMotor, rDriveMotor, lToolMotor, rToolMotor):
        self.lDriveMotor = lDriveMotor
        self.rDriveMotor = rDriveMotor
        self.lToolMotor = lToolMotor
        self.rToolMotor = rToolMotor
        self.imu = motion_sensor
        #self.colourSens = ColorSensor(port_def.colourSensPort)
        self.buttons = button


def motorsStopCoast(motors_sensors):
    motor.stop(motors_sensors.lDriveMotor)
    motor.stop(motors_sensors.rDriveMotor)
    motor.stop(motors_sensors.lToolMotor)
    motor.stop(motors_sensors.rToolMotor)

def port_to_string(port):
    if port == port.A:
        return 'A'
    elif port == port.B:
        return 'B'
    elif port == port.C:
        return 'C'
    elif port == port.D:
        return 'D'
    elif port == port.E:
        return 'E'
    elif port == port.F:
        return 'F'



def prepare_motors_sensor(port_definitions):

    try:
        l_drive_motor = port_definitions.lDriveMotorPort
    except:
        return port_to_string(port_definitions.lDriveMotorPort)

    try:
        r_drive_motor = port_definitions.rDriveMotorPort

    except:
        return port_to_string(port_definitions.rDriveMotorPort)

    try:
        l_tool_motor = port_definitions.lToolMotorPort

    except:
        return port_to_string(port_definitions.lToolMotorPort)

    try:
        r_tool_motor = port_definitions.rToolMotorPort

    except:
        return port_to_string(port_definitions.rToolMotorPort)


    # Create MotorsSensors instance
    motors_sensors = MotorsSensors(l_drive_motor, r_drive_motor, l_tool_motor, r_tool_motor)

    return motors_sensors


def set_speed_pid(motors_sensors, speed_p, speed_i, speed_d):
    pass
    # Set PID values for motors
    #print('Current PID values for motors:', motors_sensors.lDriveMotor.control.pid())
    #motors_sensors.l_drive_motor.set_speed_pid_kp(speed_p)
    #motors_sensors.l_drive_motor.set_speed_pid_ki(speed_i)
    #motors_sensors.l_drive_motor.set_speed_pid_kd(speed_d)
    #motors_sensors.r_drive_motor.set_speed_pid_kp(speed_p)
    #motors_sensors.r_drive_motor.set_speed_pid_ki(speed_i)
    #motors_sensors.r_drive_motor.set_speed_pid_kd(speed_d)





# ==================== ReadSensors.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 



def reset_all(motors_sensors):
    motors_sensors.imu.reset_yaw(0)


def get_sensor_value(sensor_id, sensor_act_values):
    if sensor_id == DRIVEENC:
        return (sensor_act_values[LDRIVEENC] + sensor_act_values[RDRIVEENC]) / 2.0
    elif sensor_id == DRIVESPEED:
        return (sensor_act_values[LDRIVESPEED] + sensor_act_values[RDRIVESPEED]) / 2.0
    else:
        return sensor_act_values[sensor_id]



def read_sensors(motors_sensors, sensor_act_values, start_time, writer):
    

    if sensor_act_values[RDRIVEENCREAD] and sensor_act_values[LDRIVEENCREAD]:
        sensor_act_values[DRIVEMOTORENCPREV] = get_sensor_value(DRIVEENC, sensor_act_values)

    if sensor_act_values[LDRIVEENCREAD]:
        sensor_act_values[LDRIVEENC] = -motor.relative_position(motors_sensors.lDriveMotor)
    if sensor_act_values[RDRIVEENCREAD]:
        sensor_act_values[RDRIVEENC] = motor.relative_position(motors_sensors.rDriveMotor)
    if sensor_act_values[LTOOLENCREAD]:
        sensor_act_values[LTOOLENC] = motor.relative_position(motors_sensors.lToolMotor)
    if sensor_act_values[RTOOLENCREAD]:
        sensor_act_values[RTOOLENC] = motor.relative_position(motors_sensors.rToolMotor)

    if sensor_act_values[LDRIVESPEEDREAD]:
        sensor_act_values[LDRIVESPEED] = motor.velocity(motors_sensors.lDriveMotor)
    if sensor_act_values[RDRIVESPEEDREAD]:
        sensor_act_values[RDRIVESPEED] = motor.velocity(motors_sensors.rDriveMotor)
    if sensor_act_values[LTOOLSPEEDREAD]:
        sensor_act_values[LTOOLSPEED] = motor.velocity(motors_sensors.lToolMotor)
    if sensor_act_values[RTOOLSPEEDREAD]:
        sensor_act_values[RTOOLSPEED] = motor.velocity(motors_sensors.rToolMotor)

    if sensor_act_values[GYROREAD]:
        sensor_act_values[GYRO] = motors_sensors.imu.tilt_angles()[0]

        if abs(sensor_act_values[GYRO] - sensor_act_values[GYROPREV]) > 3400:
            if sensor_act_values[GYRO] > sensor_act_values[GYROPREV]:
                sensor_act_values[GYROOFFSET] -= 3600
            else:
                sensor_act_values[GYROOFFSET] += 3600

        sensor_act_values[GYROPREV] = sensor_act_values[GYRO]
        sensor_act_values[GYRO] = (sensor_act_values[GYRO] + sensor_act_values[GYROOFFSET])/10


    if sensor_act_values[RIGHTBUTTONREAD]:
        sensor_act_values[RIGHTBUTTON] = motors_sensors.buttons.pressed(2)

    if DEBUG_:
        log_csv(writer, sensor_act_values)

    
    sensor_act_values[PREVTIME] = sensor_act_values[TIME]
    sensor_act_values[TIME] = time.ticks_ms()/1000 - start_time

    # Reset actuator variables
    sensor_act_values[LDRIVEPOW] = 0.0
    sensor_act_values[RDRIVEPOW] = 0.0
    sensor_act_values[LTOOLPOW] = 0.0
    sensor_act_values[RTOOLPOW] = 0.0

    sensor_act_values[LDRIVECOR] = 0.0
    sensor_act_values[RDRIVECOR] = 0.0
    sensor_act_values[LTOOLCOR] = 0.0
    sensor_act_values[RTOOLCOR] = 0.0

    sensor_act_values[LDRIVEENCREAD] = not SPARSE_SENSOR_READING_
    sensor_act_values[RDRIVEENCREAD] = not SPARSE_SENSOR_READING_
    sensor_act_values[LTOOLENCREAD] = not SPARSE_SENSOR_READING_
    sensor_act_values[RTOOLENCREAD] = not SPARSE_SENSOR_READING_

    sensor_act_values[LDRIVESPEEDREAD] = True
    sensor_act_values[RDRIVESPEEDREAD] = True
    sensor_act_values[LTOOLSPEEDREAD] = not SPARSE_SENSOR_READING_
    sensor_act_values[RTOOLSPEEDREAD] = not SPARSE_SENSOR_READING_

    sensor_act_values[GYROREAD] = not SPARSE_SENSOR_READING_
    sensor_act_values[RIGHTBUTTONREAD] = not SPARSE_SENSOR_READING_

# ==================== RunEvents.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 


#@micropython.native
def math_func(x, func):
    if func.t == FUNC_CONST:
        return func.c

    elif func.t == FUNC_LINEAR:
        if func.step_prev < 0:
            func.step_prev = x

        result = func.m * (x - func.step_prev) + func.e
        if result > func.hb:
            return func.hb
        elif result < func.lb:
            return func.lb
        else:
            return round(result*50)/50

    elif func.t == FUNC_QUADRATIC:
        return func.a * x * x + func.b * x + func.c

#@micropython.native
def set_read_sensor(sensor_id, sensor_act_values):
    if sensor_id == GYRO:
        sensor_act_values[GYROREAD] = True

    elif sensor_id < 4:
        sensor_act_values[sensor_id+27] = True

    elif sensor_id > 20 and sensor_id < 25:
        sensor_act_values[sensor_id+10] = True
    elif sensor_id == DRIVEENC:
        sensor_act_values[LDRIVEENCREAD] = True
        sensor_act_values[RDRIVEENCREAD] = True
    elif sensor_id == DRIVESPEED:
        sensor_act_values[LDRIVESPEEDREAD] = True
        sensor_act_values[RDRIVESPEEDREAD] = True
    
   

#@micropython.native
def run_events(event_list, active_table, cond_table, sensor_act_values):
    sensor_act_values[TIME] = time.ticks_ms()/1000
    running = True
    for event in event_list:
        event_type = event.t

        if event_type == EVENT_SENSORVALUE and active_table[event.event.process_id]:
            sensor_value = get_sensor_value(event.sensor_id, sensor_act_values)
            if event.sensor_prev < -9998.0:
                event.sensor_prev = sensor_value

            if event.expr == '>':
                cond_table[event.sensvalcondid] = (sensor_value - event.sensor_prev >= event.sensor_target)
            elif event.expr == '<':
                cond_table[event.sensvalcondid] = (sensor_value - event.sensor_prev <= event.sensor_target)
            else:
                if DEBUG_:
                    print("Invalid character {} at SensorValue".format(event.expr))

            set_read_sensor(event.sensor_id, sensor_act_values)

        elif event_type == EVENT_MOTORSPEEDCONTROL and active_table[event.event.process_id]:
            motor_value = math_func(get_sensor_value(event.sensor_id, sensor_act_values), event.func)
            set_motor_pow(motor_value, event.motor_id, sensor_act_values)
            set_read_sensor(event.sensor_id, sensor_act_values)

        elif event_type == EVENT_PIDGYRO and active_table[event.event.process_id]:
            sensor_value = get_sensor_value(GYRO, sensor_act_values)
            if event.sensor_prev < -9998.0:
                event.sensor_prev = sensor_value

            event.motor_correction = compute_pid(
                sensor_value - event.sensor_prev,
                event.heading,
                sensor_act_values[TIME] - sensor_act_values[PREVTIME],
                event.pid
            )

            set_motor_pow(-event.motor_correction, LDRIVECOR, sensor_act_values)
            set_motor_pow(event.motor_correction, RDRIVECOR, sensor_act_values)
            sensor_act_values[GYROREAD] = True
            set_read_sensor(DRIVEENC, sensor_act_values)

        elif event_type == EVENT_PIDLINE and active_table[event.event.process_id]:
            event.motor_correction = compute_pid(
                get_sensor_value(COLOURSENS, sensor_act_values),
                event.brightness_target,
                sensor_act_values[TIME] - sensor_act_values[PREVTIME],
                event.pid
            )
            set_motor_pow(event.motor_correction, LDRIVECOR, sensor_act_values)
            set_motor_pow(-event.motor_correction, RDRIVECOR, sensor_act_values)

        elif event_type == EVENT_COMPUTEMOTORSTALL and active_table[event.event.process_id]:
            event.buffer.append(get_sensor_value(event.motor_id, sensor_act_values))
            if len(event.buffer) == event.buffer_size:
                avg_speed = sum(event.buffer) / len(event.buffer)
                cond_table[event.event.term_conditions_id] = (avg_speed < event.min_mov_avg_speed)
                event.buffer.pop(0)
            set_read_sensor(event.motor_id, sensor_act_values)

        elif event_type == EVENT_TIMER and active_table[event.event.process_id]:
            if event.time_prev == -1.0:
                event.time_prev = sensor_act_values[TIME]
            else:
                time_passed = sensor_act_values[TIME] - event.time_prev
                if time_passed >= event.time:
                    cond_table[event.event.term_conditions_id] = True


        elif event_type == EVENT_HALT and active_table[event.event.process_id]:
            running = False

    return running

# ==================== SpawnTerminateEvents.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 



def set_var_spawn(result, cond, active_table, cond_table, sensor_act_values):
    cond_table[cond.cond_id] = result
    if result:
        if cond.process_id != 0:
            active_table[cond.process_id] = True
            set_read_sensor(cond.sensor_needed, sensor_act_values)


def set_var_term(result, cond, active_table, terminated_table, cond_table):
    cond_table[cond.cond_id] = result
    if result and cond.process_id != 0:
        
        active_table[cond.process_id] = False
        terminated_table[cond.process_id] = True


def spawn_events(spawn_list, active_table, terminated_table, cond_table, sensor_act_values):
    for condition in spawn_list:
        cond_type = condition.t
        if cond_type == CONDITION_ISTERMINATED:
            if not cond_table[condition.cond.cond_id]:
                set_var_spawn(terminated_table[condition.watch_process_id], condition.cond, active_table, cond_table, sensor_act_values)
        elif cond_type == CONDITION_AND:
            if not cond_table[condition.cond.cond_id]:
                set_var_spawn(cond_table[condition.watch_cond_id0] and cond_table[condition.watch_cond_id1], condition.cond, active_table, cond_table, sensor_act_values)
        elif cond_type == CONDITION_OR:
            if not cond_table[condition.cond.cond_id]:
                set_var_spawn(cond_table[condition.watch_cond_id0] or cond_table[condition.watch_cond_id1], condition.cond, active_table, cond_table, sensor_act_values)
        elif cond_type == CONDITION_NOT:
            if not cond_table[condition.cond.cond_id]:
                set_var_spawn(not cond_table[condition.watch_cond_id], condition.cond, active_table, cond_table, sensor_act_values)
        elif cond_type == CONDITIONS_STARTIMMEDIATELY:
            if not cond_table[condition.cond.cond_id]:
                set_var_spawn(True, condition.cond, active_table, cond_table, sensor_act_values)


def terminate_events(term_list, active_table, terminated_table, cond_table, sensor_act_values):
    for condition in term_list:
        if active_table[condition.cond.process_id]:
            cond_type = condition.t
            if cond_type == CONDITION_ISTERMINATED:
                set_var_term(terminated_table[condition.watch_process_id], condition.cond, active_table, terminated_table, cond_table)
            elif cond_type == CONDITION_AND:
                set_var_term(cond_table[condition.watch_cond_id0] and cond_table[condition.watch_cond_id1], condition.cond, active_table, terminated_table, cond_table)
            elif cond_type == CONDITION_OR:
                set_var_term(cond_table[condition.watch_cond_id0] or cond_table[condition.watch_cond_id1], condition.cond, active_table, terminated_table, cond_table)
            elif cond_type == CONDITION_NOT:
                set_var_term(not cond_table[condition.watch_cond_id], condition.cond, active_table, terminated_table, cond_table)
            elif cond_type == CONDITION_STOPIMMEDIATELY:
                set_var_term(True, condition.cond, active_table, terminated_table, cond_table)
            elif cond_type == CONDITION_SENSORVALUE:
                set_var_term(cond_table[condition.cond.cond_id], condition.cond, active_table, terminated_table, cond_table)

# ==================== StartExecution.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 



# Initialisation RULES
# EVERY variable with *_prev must be initialized with -1!
# the process_id 0 is allocated for usage as a none pointer!


def start_execution(round_instructions_path, port_definitions, active_table, terminated_table, cond_table, notetaker):
    # Assume read_instructions is a function that has been defined to populate lists and variables
    light.color(0, color.RED)
    name, round_timeout, speed_p, speed_i, speed_d, file_forward, event_list, spawn_list, term_list = read_instructions(round_instructions_path, notetaker)

    writer = None
    if DEBUG_:
        writer = init_logger("/flash/records/{}.csv".format(name))
        log_header_csv(writer)# Assume log_header_csv is defined

    # Prepare motors and sensors
    motors_sensors = prepare_motors_sensor(port_definitions)
    if isinstance(motors_sensors, str):
        return motors_sensors

    set_speed_pid(motors_sensors, speed_p, speed_i, speed_d)

    # Ensure tables are large enough
    if len(active_table) < len(event_list) or len(terminated_table) < len(event_list) or len(cond_table) < len(spawn_list) + len(term_list):
        active_table = array.array('b', [False] * (len(event_list) + 1))
        active_table[0] = True
        terminated_table = array.array('b', [False] * (len(event_list) + 1))
        cond_table = array.array('b', [False] * (len(spawn_list) + len(term_list)))

    process_loop(spawn_list, event_list, term_list, motors_sensors, active_table, terminated_table, cond_table, writer, notetaker)


    # Reset state tables
    active_table[:] = array.array('b', [False] * len(active_table))
    active_table[0] = True
    terminated_table[:] = array.array('b', [False] * len(terminated_table))
    cond_table[:] = array.array('b', [False] * len(cond_table))

    light.color(0, color.GREEN)

    return file_forward


# ==================== ProcessLoop.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 


def terminate_process_loop(start_time, round_summary, motors_sensors, sensor_act_values, notetaker):
    # Assuming start_time is the timestamp when the process loop started
    elapsed_time = time.ticks_ms()/1000 - start_time
    print(elapsed_time)
    print(round_summary.loop_count)
    round_summary.wall_time = int(elapsed_time)
    round_summary.mean_loop_time = float(elapsed_time) / float(round_summary.loop_count)
    round_summary.mean_f = int(1.0 / round_summary.mean_loop_time)

    notetaker.log("TERMINATED PROCESS LOOP")
    notetaker.log("Note down some stats about this round ...")

    #motors_stop_coast(motors_sensors)

    round_summary.total_travelled_distance = int((sensor_act_values[LDRIVEENC] + sensor_act_values[RDRIVEENC]) / 2.0)

    notetaker.log("=========== ROUND SUMMARY ===========")
    notetaker.log("Loop count: {}".format(round_summary.loop_count))
    notetaker.log("Wall time: {}s".format(round_summary.wall_time))
    notetaker.log("Average loop time (higher means worse): {}s".format(round(round_summary.mean_loop_time, 2)))
    notetaker.log("Average loop frequency: {}Hz".format(round_summary.mean_f))
    notetaker.log("Loop time maximum (higher means worse): {}s".format(round(round_summary.max_loop_time, 2)))
    notetaker.log("Total travelled distance in motor degrees: {}".format(round_summary.total_travelled_distance))


def process_loop(spawn_list, event_list, term_list, motors_sensors, active_table, terminated_table, cond_table, writer, notetaker):
    running = True
    start_time = time.ticks_ms()/1000
    round_summary = RoundSummary()

    # Sensor Values initialization and reset
    reset_all(motors_sensors)
    sensor_act_values = {i: 0.0 for i in range(40)}

    sensor_act_values[GYROREAD] = True

    TerminateProcessLoop = terminate_process_loop
    ReadSensors = read_sensors
    RunEvents = run_events
    SpawnEvents = spawn_events
    TerminateEvents = terminate_events
    WriteToActuators = write_to_actuators
    Check = check

    while True:
        # Check if loop should continue running
        if not running:
            TerminateProcessLoop(start_time, round_summary, motors_sensors, sensor_act_values, notetaker)
            break

        # Read sensor values
        ReadSensors(motors_sensors, sensor_act_values, start_time, writer)

        # Run events
        running = RunEvents(event_list, active_table, cond_table, sensor_act_values)

        # Spawn and terminate processes
        TerminateEvents(term_list, active_table, terminated_table, cond_table, sensor_act_values)
        SpawnEvents(spawn_list, active_table, terminated_table, cond_table, sensor_act_values)

        # Write computed values to actuators
        WriteToActuators(motors_sensors, sensor_act_values)

        # Perform check
        Check(round_summary, sensor_act_values)

# ==================== main.py ====================
# LAST CHANGE WAS AT: 2024-04-25 17:03:08.006240 


def main():
    # Initialize logger
    notetaker = Notetaker('/flash/log.txt')
    light.color(0, color.GREEN)

    # Define ports
    port_definitions = PortDefinition(
        l_drive_port=port.A,
        r_drive_port=port.B,
        l_tool_port=port.C,
        r_tool_port=port.D,
        colour_port=None,
    )

    paths = []

    # Glob for instruction files
    for path in os.listdir('/flash/instructions/'):
        if '00' in path:
            paths.append('/flash/instructions/'+path)

    # Read boolean table lengths
    with open(TABLELENGTHSFILE, 'r') as file:
        contents = file.read()
    boolean_table_lengths = contents.split(";")

    # Prepare boolean tables
    active_table = array.array('b', [False] * (int(boolean_table_lengths[0]) + 1))
    active_table[0] = True
    terminated_table = array.array('b', [False] * (int(boolean_table_lengths[1]) + 1))
    cond_table = array.array('b', [False] * (int(boolean_table_lengths[1]) + int(boolean_table_lengths[2])))

    index = 0
    index_prev = -1
    while True:

        if button.pressed(button.LEFT):
            print('start')
            notetaker.log("Starting execution!")
            result = start_execution(paths[index], port_definitions, active_table, terminated_table, cond_table, notetaker)
            if result == 0 and index < len(paths) - 1:
                index += 1

        if button.pressed(button.RIGHT) and index < len(paths) - 1:
            index += 1
            

        #if Button.LEFT in pressed and index > 0:
        #    index -= 1
            
        if index != index_prev:
            light_matrix.write(str(index+1))
            index_prev = index

if __name__ == "__main__":
    main()

