from consts import *; from Logger import log_csv; import time, motor


def reset_all(motors_sensors):
    motors_sensors.imu.reset_yaw(0)


def get_sensor_value(sensor_id, sensor_act_values):
    if sensor_id == DRIVEENC:
        return (sensor_act_values[LDRIVEENC] + sensor_act_values[RDRIVEENC]) / 2.0
    elif sensor_id == DRIVESPEED:
        return (sensor_act_values[LDRIVESPEED] + sensor_act_values[RDRIVESPEED]) / 2.0
    else:
        return sensor_act_values[sensor_id]



def read_sensors(motors_sensors, sensor_act_values, start_time, writer):
    

    if sensor_act_values[RDRIVEENCREAD] and sensor_act_values[LDRIVEENCREAD]:
        sensor_act_values[DRIVEMOTORENCPREV] = get_sensor_value(DRIVEENC, sensor_act_values)

    if sensor_act_values[LDRIVEENCREAD]:
        sensor_act_values[LDRIVEENC] = -motor.relative_position(motors_sensors.lDriveMotor)
    if sensor_act_values[RDRIVEENCREAD]:
        sensor_act_values[RDRIVEENC] = motor.relative_position(motors_sensors.rDriveMotor)
    if sensor_act_values[LTOOLENCREAD]:
        sensor_act_values[LTOOLENC] = motor.relative_position(motors_sensors.lToolMotor)
    if sensor_act_values[RTOOLENCREAD]:
        sensor_act_values[RTOOLENC] = motor.relative_position(motors_sensors.rToolMotor)

    if sensor_act_values[LDRIVESPEEDREAD]:
        sensor_act_values[LDRIVESPEED] = motor.velocity(motors_sensors.lDriveMotor)
    if sensor_act_values[RDRIVESPEEDREAD]:
        sensor_act_values[RDRIVESPEED] = motor.velocity(motors_sensors.rDriveMotor)
    if sensor_act_values[LTOOLSPEEDREAD]:
        sensor_act_values[LTOOLSPEED] = motor.velocity(motors_sensors.lToolMotor)
    if sensor_act_values[RTOOLSPEEDREAD]:
        sensor_act_values[RTOOLSPEED] = motor.velocity(motors_sensors.rToolMotor)

    if sensor_act_values[GYROREAD]:
        sensor_act_values[GYRO] = motors_sensors.imu.tilt_angles()[0]

        if abs(sensor_act_values[GYRO] - sensor_act_values[GYROPREV]) > 3400:
            if sensor_act_values[GYRO] > sensor_act_values[GYROPREV]:
                sensor_act_values[GYROOFFSET] -= 3600
            else:
                sensor_act_values[GYROOFFSET] += 3600

        sensor_act_values[GYROPREV] = sensor_act_values[GYRO]
        sensor_act_values[GYRO] = (sensor_act_values[GYRO] + sensor_act_values[GYROOFFSET])/10


    if sensor_act_values[RIGHTBUTTONREAD]:
        sensor_act_values[RIGHTBUTTON] = motors_sensors.buttons.pressed(2)

    if DEBUG_:
        log_csv(writer, sensor_act_values)

    
    sensor_act_values[PREVTIME] = sensor_act_values[TIME]
    sensor_act_values[TIME] = time.ticks_ms()/1000 - start_time

    # Reset actuator variables
    sensor_act_values[LDRIVEPOW] = 0.0
    sensor_act_values[RDRIVEPOW] = 0.0
    sensor_act_values[LTOOLPOW] = 0.0
    sensor_act_values[RTOOLPOW] = 0.0

    sensor_act_values[LDRIVECOR] = 0.0
    sensor_act_values[RDRIVECOR] = 0.0
    sensor_act_values[LTOOLCOR] = 0.0
    sensor_act_values[RTOOLCOR] = 0.0

    sensor_act_values[LDRIVEENCREAD] = not SPARSE_SENSOR_READING_
    sensor_act_values[RDRIVEENCREAD] = not SPARSE_SENSOR_READING_
    sensor_act_values[LTOOLENCREAD] = not SPARSE_SENSOR_READING_
    sensor_act_values[RTOOLENCREAD] = not SPARSE_SENSOR_READING_

    sensor_act_values[LDRIVESPEEDREAD] = True
    sensor_act_values[RDRIVESPEEDREAD] = True
    sensor_act_values[LTOOLSPEEDREAD] = not SPARSE_SENSOR_READING_
    sensor_act_values[RTOOLSPEEDREAD] = not SPARSE_SENSOR_READING_

    sensor_act_values[GYROREAD] = not SPARSE_SENSOR_READING_
    sensor_act_values[RIGHTBUTTONREAD] = not SPARSE_SENSOR_READING_