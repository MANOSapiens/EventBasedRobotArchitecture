from consts import *; from PID import compute_pid; from ReadSensors import get_sensor_value; from Actuators import set_motor_pow; import time

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