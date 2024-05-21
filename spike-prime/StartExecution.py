from consts import *; from ProcessLoop import process_loop; from Port import prepare_motors_sensor, set_speed_pid; from Logger import log_header_csv, init_logger; from ReadInstructions import read_instructions; import csv, array


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
