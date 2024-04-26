from pybricks.pupdevices import Motor
from pybricks.parameters import Port


class PortDefinition:
    def __init__(self, l_drive_port, r_drive_port, l_tool_port, r_tool_port, colour_port):
        self.lDriveMotorPort = l_drive_port
        self.rDriveMotorPort = r_drive_port
        self.lToolMotorPort = l_tool_port
        self.rToolMotorPort = r_tool_port
        self.colourSensPort = colour_port

class MotorsSensors:
    def __init__(self, hub, lDriveMotor, rDriveMotor, lToolMotor, rToolMotor):
        self.hub = hub
        self.lDriveMotor = lDriveMotor
        self.rDriveMotor = rDriveMotor
        self.lToolMotor = lToolMotor
        self.rToolMotor = rToolMotor
        self.imu = self.hub.imu
        #self.colourSens = ColorSensor(port_def.colourSensPort)
        self.buttons = self.hub.buttons

    
def motorsStopCoast(motors_sensors):
    motors_sensors.lDriveMotor.stop()
    motors_sensors.rDriveMotor.stop()
    motors_sensors.lToolMotor.stop()
    motors_sensors.rToolMotor.stop()

def port_to_string(port):
    if port == Port.A:
        return 'A'
    elif port == Port.B:
        return 'B'
    elif port == Port.C:
        return 'C'
    elif port == Port.D:
        return 'D'
    elif port == Port.E:
        return 'E'
    elif port == Port.F:
        return 'F'
    


def prepare_motors_sensor(hub, port_definitions):
    
    try:
        l_drive_motor = Motor(port_definitions.lDriveMotorPort, reset_angle=False, profile=3)#
    except:
        return port_to_string(port_definitions.lDriveMotorPort)
    
    try:
        r_drive_motor = Motor(port_definitions.rDriveMotorPort, reset_angle=False, profile=3)

    except:
        return port_to_string(port_definitions.rDriveMotorPort)
    
    try:
        l_tool_motor = Motor(port_definitions.lToolMotorPort, reset_angle=False, profile=3)

    except:
        return port_to_string(port_definitions.lToolMotorPort)
    
    try:
        r_tool_motor = Motor(port_definitions.rToolMotorPort, reset_angle=False, profile=3)

    except:
        return port_to_string(port_definitions.rToolMotorPort)


    # Create MotorsSensors instance
    motors_sensors = MotorsSensors(hub, l_drive_motor, r_drive_motor, l_tool_motor, r_tool_motor)

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