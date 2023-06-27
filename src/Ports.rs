extern crate ev3dev_lang_rust;

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