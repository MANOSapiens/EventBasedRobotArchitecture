from spike_prime.consts import *


class A: # EventID
    def __init__(self, process_id: int, spawn_conditions_id: int, term_conditions_id: int):
        self.process_id = process_id
        self.spawn_conditions_id = spawn_conditions_id
        self.term_conditions_id = term_conditions_id

class B: # CondID
    def __init__(self, process_id: int, cond_id: int, sensor_needed: int):
        self.process_id = process_id
        self.cond_id = cond_id
        self.sensor_needed = sensor_needed

class C: # PID
    def __init__(self, p: float, i: float, d: float, max_i: float):
        self.p = p
        self.i = i
        self.d = d
        self.max_i = max_i
        self.sum_i = 0
        self.prev_e = -1


class D: #ConstFunc
    def __init__(self, c: float):
        self.c = c
        self.t = FUNC_CONST

class E: #LinearFunc
    def __init__(self, m: float, e: float, lb: float, hb: float):
        self.m = m
        self.e = e
        self.lb = lb
        self.hb = hb
        self.step_prev = -1
        self.t = FUNC_LINEAR

class F: # Placeholder
    def __init__(self, event: A):
        self.event = event

class G: # SensorValue
    def __init__(self, event: A, sensor_target: float, sensor_prev: float, sensor_id: int, expr: str, sensvalcondid: int):
        self.event = event
        self.sensor_target = sensor_target
        self.sensor_prev = sensor_prev
        self.sensor_id = sensor_id
        self.expr = expr
        self.sensvalcondid = sensvalcondid
        self.t = EVENT_SENSORVALUE

class H: # Motorspeedcontrol
    def __init__(self, event: A, motor_id: int, sensor_id: int, func):
        self.event = event
        self.motor_id = motor_id
        self.sensor_id = sensor_id
        self.func = func
        self.t = EVENT_MOTORSPEEDCONTROL

class I: # PIDGyro
    def __init__(self, event: A, heading: float, pid: C, sensor_prev: float):
        self.event = event
        self.heading = heading
        self.pid = pid
        self.sensor_prev = sensor_prev
        self.t = EVENT_PIDGYRO

class J: # Timer
    def __init__(self, event: A, time: float, time_prev: float):
        self.event = event
        self.time = time
        self.time_prev = time_prev
        self.t = EVENT_TIMER

class K: # ComputeMotorStall
    def __init__(self, event: A, motor_id: int):
        self.event = event
        self.motor_id = motor_id
        self.t = EVENT_COMPUTEMOTORSTALL

class L: # Halt
    def __init__(self, event: A):
        self.event = event
        self.t = EVENT_HALT


class M: # PlaceholderCond
    def __init__(self, cond: B):
        self.cond = cond
        self.t = CONDITION_PLACEHOLDER


class N: # IsTerminated
    def __init__(self, cond: B, watch_process_id: int):
        self.cond = cond
        self.watch_process_id = watch_process_id
        self.t = CONDITION_ISTERMINATED


class O: # And
    def __init__(self, cond: B, watch_cond_id0: int, watch_cond_id1: int):
        self.cond = cond
        self.watch_cond_id0 = watch_cond_id0
        self.watch_cond_id1 = watch_cond_id1
        self.t = CONDITION_AND


class P: #Or
    def __init__(self, cond: B, watch_cond_id0: int, watch_cond_id1: int):
        self.cond = cond
        self.watch_cond_id0 = watch_cond_id0
        self.watch_cond_id1 = watch_cond_id1
        self.t = CONDITION_OR


class Q: # Not
    def __init__(self, cond: B, watch_cond_id: int):
        self.cond = cond
        self.watch_cond_id = watch_cond_id
        self.t = CONDITION_NOT


class R: # StartImmediately
    def __init__(self, cond: B):
        self.cond = cond
        self.t = CONDITIONS_STARTIMMEDIATELY


class S: # StopImmediately
    def __init__(self, cond: B):
        self.cond = cond
        self.t = CONDITION_STOPIMMEDIATELY



class T: # SensorValueCond
    def __init__(self, cond: B):
        self.cond = cond
        self.t = CONDITION_SENSORVALUE
