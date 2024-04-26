import array
from consts import *


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

class IsTerminated:
    def __init__(self, cond: CondID, watch_process_id: int):
        self.cond = cond
        self.watch_process_id = watch_process_id
        self.t = CONDITION_ISTERMINATED

class And:
    def __init__(self, cond: CondID, watch_cond_id0: int, watch_cond_id1: int):
        self.cond = cond
        self.watch_cond_id0 = watch_cond_id0
        self.watch_cond_id1 = watch_cond_id1
        self.t = CONDITION_AND

class Or:
    def __init__(self, cond: CondID, watch_cond_id0: int, watch_cond_id1: int):
        self.cond = cond
        self.watch_cond_id0 = watch_cond_id0
        self.watch_cond_id1 = watch_cond_id1
        self.t = CONDITION_OR

class Not:
    def __init__(self, cond: CondID, watch_cond_id: int):
        self.cond = cond
        self.watch_cond_id = watch_cond_id
        self.t = CONDITION_NOT

class StartImmediately:
    def __init__(self, cond: CondID):
        self.cond = cond
        self.t = CONDITIONS_STARTIMMEDIATELY

class StopImmediately:
    def __init__(self, cond: CondID):
        self.cond = cond
        self.t = CONDITION_STOPIMMEDIATELY

class SensorValueCond:
    def __init__(self, cond: CondID):
        self.cond = cond
        self.t = CONDITION_SENSORVALUE
