from consts import *
from RunEvents import set_read_sensor

def set_var_spawn(result, cond, active_table, cond_table, sensor_act_values):
    cond_table[cond.cond_id] = result
    if result:
        print('Spawned', cond.process_id)
        if cond.process_id != 0:
            active_table[cond.process_id] = True
            set_read_sensor(cond.sensor_needed, sensor_act_values)


def set_var_term(result, cond, active_table, terminated_table, cond_table):
    cond_table[cond.cond_id] = result
    if result and cond.process_id != 0:
        print('Terminated', cond.process_id)
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
        cond_type = condition.t
        if cond_type == CONDITION_ISTERMINATED and active_table[condition.cond.process_id]:
            set_var_term(terminated_table[condition.watch_process_id], condition.cond, active_table, terminated_table, cond_table)
        elif cond_type == CONDITION_AND and active_table[condition.cond.process_id]:
            set_var_term(cond_table[condition.watch_cond_id0] and cond_table[condition.watch_cond_id1], condition.cond, active_table, terminated_table, cond_table)
        elif cond_type == CONDITION_OR and active_table[condition.cond.process_id]:
            set_var_term(cond_table[condition.watch_cond_id0] or cond_table[condition.watch_cond_id1], condition.cond, active_table, terminated_table, cond_table)
        elif cond_type == CONDITION_NOT and active_table[condition.cond.process_id]:
            set_var_term(not cond_table[condition.watch_cond_id], condition.cond, active_table, terminated_table, cond_table)
        elif cond_type == CONDITION_STOPIMMEDIATELY and active_table[condition.cond.process_id]:
            set_var_term(True, condition.cond, active_table, terminated_table, cond_table)
        elif cond_type == CONDITION_SENSORVALUE and active_table[condition.cond.process_id]:
            set_var_term(cond_table[condition.cond.cond_id], condition.cond, active_table, terminated_table, cond_table)
