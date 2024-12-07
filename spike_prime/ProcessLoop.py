from consts import *; from Check import RoundSummary; from ReadSensors import read_sensors, reset_all; from RunEvents import run_events; from SpawnTerminateEvents import spawn_events, terminate_events; from Actuators import write_to_actuators; from Check import check; import time

def terminate_process_loop(start_time, round_summary, motors_sensors, sensor_act_values, notetaker):
    # Assuming start_time is the timestamp when the process loop started
    elapsed_time = time.ticks_ms()/1000 - start_time
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
        