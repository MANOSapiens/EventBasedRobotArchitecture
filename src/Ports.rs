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
    pub colourSensPort0: SensorPort,
    pub colourSensPort1: SensorPort
}

/* pub struct MotorsSensors {
    pub lDriveMotor: LargeMotor,
    pub rDriveMotor: LargeMotor,
    pub lToolMotor: MediumMotor,
    pub rToolMotor: MediumMotor,
    pub gyroSens: GyroSensor,
    pub colourSens0: ColorSensor,
    pub colourSens1: ColorSensor
} */

pub struct MotorsSensors {
    
}

pub fn prepare_motors_sensor(port_definitions: PortDefinition) -> MotorsSensors {
    // Try to init all motors and sensors
    // Panics (throws error) if not available

    /* let mut motors_sensors = MotorsSensors {
        lDriveMotor: LargeMotor::get(port_definitions.lDriveMotorPort)
            .expect("failed to load motor or sensor"),
        rDriveMotor: LargeMotor::get(port_definitions.rDriveMotorPort)
            .expect("failed to load motor or sensor"),
        lToolMotor: MediumMotor::get(port_definitions.lToolMotorPort)
            .expect("failed to load motor or sensor"),
        rToolMotor: MediumMotor::get(port_definitions.rToolMotorPort)
            .expect("failed to load motor or sensor"),
        gyroSens: GyroSensor::get(port_definitions.gyroSensPort)
            .expect("failed to load motor or sensor"),
        colourSens0: ColorSensor::get(port_definitions.colourSensPort0)
            .expect("failed to load motor or sensor"),
        colourSens1: ColorSensor::get(port_definitions.colourSensPort1)
            .expect("failed to load motor or sensor"),
    }; */

    let mut motors_sensors = MotorsSensors {

    };

    motors_sensors
}