from consts import *



def set_motor_pow(motor_pow, motor_id, sensor_act_values):
    if motor_id == LDRIVEPOW:
        sensor_act_values.lDriveMotorPow = motor_pow
    elif motor_id == RDRIVEPOW:
        sensor_act_values.rDriveMotorPow = motor_pow
    elif motor_id == LTOOLPOW:
        sensor_act_values.lToolMotorPow = motor_pow
    elif motor_id == RTOOLPOW:
        sensor_act_values.rToolMotorPow = motor_pow
    elif motor_id == LDRIVECOR:
        sensor_act_values.lDriveMotorCor = motor_pow
    elif motor_id == RDRIVECOR:
        sensor_act_values.rDriveMotorCor = motor_pow
    elif motor_id == LTOOLCOR:
        sensor_act_values.lToolMotorCor = motor_pow
    elif motor_id == RTOOLCOR:
        sensor_act_values.rToolMotorCor = motor_pow
    else:
        if DEBUG:
            print(f"Motor ID {motor_id} unknown while assigning a power through setMotorPow()")


def constrain_actuator_values(value):
    if value > 1.0:
        value = 1.0
    elif value < -1.0:
        value = -1.0
    return value


def write_to_actuators(motors_sensors, sensor_act_values):
    lDriveMotorPow = sensor_act_values.lDriveMotorPow + sensor_act_values.lDriveMotorCor
    rDriveMotorPow = sensor_act_values.rDriveMotorPow + sensor_act_values.rDriveMotorCor
    lToolMotorPow = sensor_act_values.lToolMotorPow + sensor_act_values.lToolMotorCor
    rToolMotorPow = sensor_act_values.rToolMotorPow + sensor_act_values.rToolMotorCor

    if lDriveMotorPow != sensor_act_values.lDriveMotorPowPrev:
        lDriveMotorPow = constrain_actuator_values(lDriveMotorPow)
        if lDriveMotorPow == 0.0:
            motors_sensors.lDriveMotor.brake()
        else:
            motors_sensors.lDriveMotor.run(lDriveMotorPow * 1000)
        sensor_act_values.lDriveMotorPowPrev = lDriveMotorPow

    if rDriveMotorPow != sensor_act_values.rDriveMotorPowPrev:
        rDriveMotorPow = constrain_actuator_values(rDriveMotorPow)
        if rDriveMotorPow == 0.0:
            motors_sensors.rDriveMotor.brake()
        else:
            motors_sensors.rDriveMotor.run(rDriveMotorPow * 1000)
        sensor_act_values.rDriveMotorPowPrev = rDriveMotorPow

    if lToolMotorPow != sensor_act_values.lToolMotorPowPrev:
        lToolMotorPow = constrain_actuator_values(lToolMotorPow)
        motors_sensors.lToolMotor.dc(lToolMotorPow * 100)
        sensor_act_values.lToolMotorPowPrev = lToolMotorPow

    if rToolMotorPow != sensor_act_values.rToolMotorPowPrev:
        rToolMotorPow = constrain_actuator_values(rToolMotorPow)
        motors_sensors.rToolMotor.dc(rToolMotorPow * 100)
        sensor_act_values.rToolMotorPowPrev = rToolMotorPow
