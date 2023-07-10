extern crate ev3dev_lang_rust;

use super::DEBUG;
use ev3dev_lang_rust::motors::{MediumMotor, LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{SensorPort, ColorSensor, GyroSensor};

pub struct PortDefinition {
    pub lDriveMotorPort: MotorPort,
    pub rDriveMotorPort: MotorPort,
    pub lToolMotorPort: MotorPort,
    pub rToolMotorPort: MotorPort,
    pub gyroSensPort: SensorPort,
    pub colourSensPort: SensorPort
}

pub struct MotorsSensors {
    pub lDriveMotor: LargeMotor,
    pub rDriveMotor: LargeMotor,
    pub lToolMotor: MediumMotor,
    pub rToolMotor: MediumMotor,
    pub gyroSens: GyroSensor,
    pub colourSens: ColorSensor
}

/* pub struct MotorsSensors {
    
} */

fn motorsRunDirect(motors_sensors: &mut MotorsSensors) {
    let _ = motors_sensors.lDriveMotor.run_direct(); //SET RUN DIRECT MODE 
    let _ = motors_sensors.rDriveMotor.run_direct(); //SET RUN DIRECT MODE
    let _ = motors_sensors.lToolMotor.run_direct(); //SET RUN DIRECT MODE
    let _ = motors_sensors.rToolMotor.run_direct(); //SET RUN DIRECT MODE

    let _ = motors_sensors.lDriveMotor.set_stop_action("brake"); //SET BRAKE MODE TO PASSIVE ELECTRICAL BRAKE
    let _ = motors_sensors.rDriveMotor.set_stop_action("brake"); //SET BRAKE MODE TO PASSIVE ELECTRICAL BRAKE
    let _ = motors_sensors.lToolMotor.set_stop_action("brake"); //SET BRAKE MODE TO PASSIVE ELECTRICAL BRAKE
    let _ = motors_sensors.rToolMotor.set_stop_action("brake"); //SET BRAKE MODE TO PASSIVE ELECTRICAL BRAKE
}

pub fn motorsStopCoast(motors_sensors: &mut MotorsSensors) {
    let _ = motors_sensors.lDriveMotor.set_stop_action("coast"); //SET BRAKE MODE TO roll to stop
    let _ = motors_sensors.rDriveMotor.set_stop_action("coast"); //SET BRAKE MODE TO roll to stop
    let _ = motors_sensors.lToolMotor.set_stop_action("coast"); //SET BRAKE MODE TO roll to stop
    let _ = motors_sensors.rToolMotor.set_stop_action("coast"); //SET BRAKE MODE TO roll to stop

    // Stop all actuators
    let _ = motors_sensors.lDriveMotor.stop();
    let _ = motors_sensors.rDriveMotor.stop();
    let _ = motors_sensors.lToolMotor.stop();
    let _ = motors_sensors.rToolMotor.stop();
}

pub fn prepare_motors_sensor(port_definitions: PortDefinition) -> MotorsSensors {
    // Try to init all motors and sensors
    // Panics (throws error) if not available

    let mut motors_sensors = MotorsSensors {
        lDriveMotor: LargeMotor::get(port_definitions.lDriveMotorPort)
            .expect("failed to load lDriveMotor"),
        rDriveMotor: LargeMotor::get(port_definitions.rDriveMotorPort)
            .expect("failed to load rDriveMotor"),
        lToolMotor: MediumMotor::get(port_definitions.lToolMotorPort)
            .expect("failed to load lToolMotor"),
        rToolMotor: MediumMotor::get(port_definitions.rToolMotorPort)
            .expect("failed to load rToolMotor"),
        gyroSens: GyroSensor::get(port_definitions.gyroSensPort)
            .expect("failed to load Gyro"),
        colourSens: ColorSensor::get(port_definitions.colourSensPort)
            .expect("failed to load colour sensor")
    };

    /* let mut motors_sensors = MotorsSensors {
    }; */

    motorsRunDirect(&mut motors_sensors);
    motors_sensors.lDriveMotor.set_position(0);
    motors_sensors.rDriveMotor.set_position(0);
    motors_sensors.lToolMotor.set_position(0);
    motors_sensors.rToolMotor.set_position(0);

    let _ = motors_sensors.gyroSens.set_mode_gyro_ang();
    let _ = motors_sensors.colourSens.set_mode_col_reflect();
    motors_sensors
}