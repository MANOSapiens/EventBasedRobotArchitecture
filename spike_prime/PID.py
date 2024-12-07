from math import sin, radians

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
