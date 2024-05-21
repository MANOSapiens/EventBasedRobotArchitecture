from consts import *; import motor



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