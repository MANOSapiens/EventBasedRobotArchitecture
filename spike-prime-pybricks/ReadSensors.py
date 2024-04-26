from consts import *
from Logger import log_csv

import time
import csv
from pybricks.parameters import Button


def reset_all(motors_sensors):
    motors_sensors.lDriveMotor.reset_angle(0)
    motors_sensors.rDriveMotor.reset_angle(0)
    motors_sensors.lToolMotor.reset_angle(0)
    motors_sensors.rToolMotor.reset_angle(0)
    motors_sensors.imu.reset_heading(0)

def get_sensor_value(sensor_id, sensor_act_values):
    if sensor_id == DRIVEENC:
        return (sensor_act_values.lDriveMotorEnc + sensor_act_values.rDriveMotorEnc) / 2.0
    elif sensor_id == LDRIVEENC:
        return sensor_act_values.lDriveMotorEnc
    elif sensor_id == RDRIVEENC:
        return sensor_act_values.rDriveMotorEnc
    elif sensor_id == LTOOLENC:
        return sensor_act_values.lToolMotorEnc
    elif sensor_id == RTOOLENC:
        return sensor_act_values.rToolMotorEnc
    elif sensor_id == DRIVESPEED:
        return (sensor_act_values.lDriveMotorSpeed + sensor_act_values.rDriveMotorSpeed) / 2.0
    elif sensor_id == LDRIVESPEED:
        return sensor_act_values.lDriveMotorSpeed
    elif sensor_id == RDRIVESPEED:
        return sensor_act_values.rDriveMotorSpeed
    elif sensor_id == LTOOLSPEED:
        return sensor_act_values.lToolMotorSpeed
    elif sensor_id == RTOOLSPEED:
        return sensor_act_values.rToolMotorSpeed
    elif sensor_id == COLOURSENS:
        return sensor_act_values.colourSensValue
    elif sensor_id == GYRO:
        return sensor_act_values.gyroAngValue
    elif sensor_id == LDRIVEPOW:
        return sensor_act_values.lDriveMotorPow
    elif sensor_id == RDRIVEPOW:
        return sensor_act_values.rDriveMotorPow
    elif sensor_id == LTOOLPOW:
        return sensor_act_values.lToolMotorPow
    elif sensor_id == RTOOLPOW:
        return sensor_act_values.rToolMotorPow
    elif sensor_id == LDRIVECOR:
        return sensor_act_values.lDriveMotorCor
    elif sensor_id == RDRIVECOR:
        return sensor_act_values.rDriveMotorCor
    elif sensor_id == LTOOLCOR:
        return sensor_act_values.lToolMotorCor
    elif sensor_id == RTOOLCOR:
        return sensor_act_values.rToolMotorCor
    elif sensor_id == RIGHTBUTTON:
        return sensor_act_values.rightButton
    elif sensor_id == TIME:
        return sensor_act_values.currentTime
    elif sensor_id == PREVTIME:
        return sensor_act_values.timePrev
    else:
        if DEBUG:
            print(f"Sensor ID {sensor_id} unknown while searching a value through getSensorValue()")
        return 0




def read_sensors(motors_sensors, sensor_act_values, start_time, writer):
    sensor_act_values.gyroRate = 0.0

    if sensor_act_values.rDriveMotorEncRead and sensor_act_values.lDriveMotorEncRead:
        sensor_act_values.driveMotorEncPrev = get_sensor_value(DRIVEENC, sensor_act_values)

    if sensor_act_values.lDriveMotorEncRead:
        sensor_act_values.lDriveMotorEnc = motors_sensors.lDriveMotor.angle()
    if sensor_act_values.rDriveMotorEncRead:
        sensor_act_values.rDriveMotorEnc = motors_sensors.rDriveMotor.angle()
    if sensor_act_values.lToolMotorEncRead:
        sensor_act_values.lToolMotorEnc = motors_sensors.lToolMotor.angle()
    if sensor_act_values.rToolMotorEncRead:
        sensor_act_values.rToolMotorEnc = motors_sensors.rToolMotor.angle()

    if sensor_act_values.lDriveMotorSpeedRead:
        sensor_act_values.lDriveMotorSpeed = motors_sensors.lDriveMotor.speed(window=SPEED_WINDOW)
    if sensor_act_values.rDriveMotorSpeedRead:
        sensor_act_values.rDriveMotorSpeed = motors_sensors.rDriveMotor.speed(window=SPEED_WINDOW)
    if sensor_act_values.lToolMotorSpeedRead:
        sensor_act_values.lToolMotorSpeed = motors_sensors.lToolMotor.speed(window=SPEED_WINDOW)
    if sensor_act_values.rToolMotorSpeedRead:
        sensor_act_values.rToolMotorSpeed = motors_sensors.rToolMotor.speed(window=SPEED_WINDOW)

    if sensor_act_values.gyroRead:
        sensor_act_values.gyroAngValue = motors_sensors.imu.heading()

    if sensor_act_values.rightButtonRead:
        sensor_act_values.rightButton = Button.RIGHT in motors_sensors.buttons.pressed()

    if DEBUG:
        log_csv(writer, sensor_act_values)
        

    sensor_act_values.timePrev = sensor_act_values.currentTime
    sensor_act_values.currentTime = time.time() - start_time

    # Reset actuator variables
    sensor_act_values.lDriveMotorPow = 0.0
    sensor_act_values.rDriveMotorPow = 0.0
    sensor_act_values.lToolMotorPow = 0.0
    sensor_act_values.rToolMotorPow = 0.0

    sensor_act_values.lDriveMotorCor = 0.0
    sensor_act_values.rDriveMotorCor = 0.0
    sensor_act_values.lToolMotorCor = 0.0
    sensor_act_values.rToolMotorCor = 0.0

    sensor_act_values.lDriveMotorEncRead = not SPARSE_SENSOR_READING
    sensor_act_values.rDriveMotorEncRead = not SPARSE_SENSOR_READING
    sensor_act_values.lToolMotorEncRead = not SPARSE_SENSOR_READING
    sensor_act_values.rToolMotorEncRead = not SPARSE_SENSOR_READING

    sensor_act_values.lDriveMotorSpeedRead = True
    sensor_act_values.rDriveMotorSpeedRead = True
    sensor_act_values.lToolMotorSpeedRead = not SPARSE_SENSOR_READING
    sensor_act_values.rToolMotorSpeedRead = not SPARSE_SENSOR_READING

    sensor_act_values.gyroRead = not SPARSE_SENSOR_READING
    sensor_act_values.rightButtonRead = not SPARSE_SENSOR_READING

