from consts import *
from Check import RoundSummary
from ReadSensors import read_sensors, reset_all
from RunEvents import run_events
from SpawnTerminateEvents import spawn_events, terminate_events
from Actuators import write_to_actuators
from Check import check

import time



class SensorActuatorValues:
    def __init__(self):
        # Motor encoders
        self.lDriveMotorEnc = 0.0
        self.rDriveMotorEnc = 0.0
        self.lToolMotorEnc = 0.0
        self.rToolMotorEnc = 0.0

        self.lDriveMotorEncRead = False
        self.rDriveMotorEncRead = False
        self.lToolMotorEncRead = False
        self.rToolMotorEncRead = False

        # Motor speeds
        self.lDriveMotorSpeed = 0.0
        self.rDriveMotorSpeed = 0.0
        self.lToolMotorSpeed = 0.0
        self.rToolMotorSpeed = 0.0

        self.lDriveMotorSpeedRead = False
        self.rDriveMotorSpeedRead = False
        self.lToolMotorSpeedRead = False
        self.rToolMotorSpeedRead = False

        # Gyroscopic sensor values
        self.colourSensValue = 0.0
        self.gyroAngValue = 0.0
        self.gyroRate = 0.0
        self.gyroRead = False

        # Motor power
        self.lDriveMotorPow = 0.0
        self.rDriveMotorPow = 0.0
        self.lToolMotorPow = 0.0
        self.rToolMotorPow = 0.0

        # Previous motor power
        self.lDriveMotorPowPrev = 0.0
        self.rDriveMotorPowPrev = 0.0
        self.lToolMotorPowPrev = 0.0
        self.rToolMotorPowPrev = 0.0
        self.driveMotorEncPrev = 0.0

        # Motor power corrections by PIDs
        self.lDriveMotorCor = 0.0
        self.rDriveMotorCor = 0.0
        self.lToolMotorCor = 0.0
        self.rToolMotorCor = 0.0

        # Button state
        self.rightButton = 0
        self.rightButtonRead = False

        # Time tracking
        self.currentTime = 0.0
        self.timePrev = -1.0


def terminate_process_loop(start_time, round_summary, motors_sensors, sensor_act_values, notetaker):
    # Assuming start_time is the timestamp when the process loop started
    elapsed_time = time.time() - start_time
    round_summary.wall_time = int(elapsed_time)
    round_summary.mean_loop_time = elapsed_time / round_summary.loop_count
    round_summary.mean_f = int(1.0 / round_summary.mean_loop_time)

    notetaker.log("TERMINATED PROCESS LOOP")
    notetaker.log("Note down some stats about this round ...")

    #motors_stop_coast(motors_sensors)

    round_summary.total_travelled_distance = int((sensor_act_values.lDriveMotorEnc + sensor_act_values.rDriveMotorEnc) / 2.0)

    notetaker.log("=========== ROUND SUMMARY ===========")
    notetaker.log(f"Loop count: {round_summary.loop_count}")
    notetaker.log(f"Wall time: {round_summary.wall_time}s")
    notetaker.log(f"Average loop time (higher means worse): {round_summary.mean_loop_time:.2f}s")
    notetaker.log(f"Average loop frequency: {round_summary.mean_f}Hz")
    notetaker.log(f"Loop time maximum (higher means worse): {round_summary.max_loop_time:.2f}s")
    notetaker.log(f"Total travelled distance in motor degrees: {round_summary.total_travelled_distance}")


def process_loop(spawn_list, event_list, term_list, motors_sensors, active_table, terminated_table, cond_table, writer, notetaker):
    running = True
    start_time = time.time()
    round_summary = RoundSummary()

    # Sensor Values initialization and reset
    reset_all(motors_sensors)
    sensor_act_values = SensorActuatorValues()

    sensor_act_values.gyroRead = True

    while True:
        # Check if loop should continue running
        if not running:
            terminate_process_loop(start_time, round_summary, motors_sensors, sensor_act_values, notetaker)
            break

        # Read sensor values
        read_sensors(motors_sensors, sensor_act_values, start_time, writer)
        
        # Run events
        running = run_events(event_list, active_table, cond_table, sensor_act_values)
        
        # Spawn and terminate processes
        terminate_events(term_list, active_table, terminated_table, cond_table, sensor_act_values)
        spawn_events(spawn_list, active_table, terminated_table, cond_table, sensor_act_values)

        # Write computed values to actuators
        write_to_actuators(motors_sensors, sensor_act_values)
        
        # Perform check
        check(round_summary, sensor_act_values)


