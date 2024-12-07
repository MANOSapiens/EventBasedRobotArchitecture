
class PortDefinition:
    def __init__(self, l_drive_port, r_drive_port, l_tool_port, r_tool_port, colour_port):
        self.lDriveMotorPort = l_drive_port
        self.rDriveMotorPort = r_drive_port
        self.lToolMotorPort = l_tool_port
        self.rToolMotorPort = r_tool_port
        self.colourSensPort = colour_port

class MotorsSensors:
    def __init__(self, lDriveMotor, rDriveMotor, lToolMotor, rToolMotor):
        self.lDriveMotor = lDriveMotor
        self.rDriveMotor = rDriveMotor
        self.lToolMotor = lToolMotor
        self.rToolMotor = rToolMotor
        self.imu = motion_sensor
        #self.colourSens = ColorSensor(port_def.colourSensPort)
        self.buttons = button


def motorsStopCoast(motors_sensors):
    motor.stop(motors_sensors.lDriveMotor)
    motor.stop(motors_sensors.rDriveMotor)
    motor.stop(motors_sensors.lToolMotor)
    motor.stop(motors_sensors.rToolMotor)

def port_to_string(port):
    if port == port.A:
        return 'A'
    elif port == port.B:
        return 'B'
    elif port == port.C:
        return 'C'
    elif port == port.D:
        return 'D'
    elif port == port.E:
        return 'E'
    elif port == port.F:
        return 'F'



def prepare_motors_sensor(port_definitions):

    try:
        l_drive_motor = port_definitions.lDriveMotorPort
    except:
        return port_to_string(port_definitions.lDriveMotorPort)

    try:
        r_drive_motor = port_definitions.rDriveMotorPort

    except:
        return port_to_string(port_definitions.rDriveMotorPort)

    try:
        l_tool_motor = port_definitions.lToolMotorPort

    except:
        return port_to_string(port_definitions.lToolMotorPort)

    try:
        r_tool_motor = port_definitions.rToolMotorPort

    except:
        return port_to_string(port_definitions.rToolMotorPort)


    # Create MotorsSensors instance
    motors_sensors = MotorsSensors(l_drive_motor, r_drive_motor, l_tool_motor, r_tool_motor)

    return motors_sensors


def set_speed_pid(motors_sensors, speed_p, speed_i, speed_d):
    pass
    # Set PID values for motors
    #print('Current PID values for motors:', motors_sensors.lDriveMotor.control.pid())
    #motors_sensors.l_drive_motor.set_speed_pid_kp(speed_p)
    #motors_sensors.l_drive_motor.set_speed_pid_ki(speed_i)
    #motors_sensors.l_drive_motor.set_speed_pid_kd(speed_d)
    #motors_sensors.r_drive_motor.set_speed_pid_kp(speed_p)
    #motors_sensors.r_drive_motor.set_speed_pid_ki(speed_i)
    #motors_sensors.r_drive_motor.set_speed_pid_kd(speed_d)



