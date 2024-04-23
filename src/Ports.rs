extern crate ev3dev_lang_rust;


use ev3dev_lang_rust::Button;
use ev3dev_lang_rust::motors::{MediumMotor, LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{SensorPort, ColorSensor, GyroSensor};
use log::{info, error};
use super::{
    COLOURSENS, DEBUG, GYRO, LDRIVECOR,
    RDRIVECOR, LDRIVEENC, RDRIVEENC, LTOOLENC, RTOOLENC, RIGHTBUTTON, DRIVEENC, LDRIVESPEED, RDRIVESPEED, LTOOLSPEED, RTOOLSPEED, DRIVESPEED, TIME, PREVTIME
};

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
    pub colourSens: ColorSensor,
    pub button: Button
}

/* pub struct MotorsSensors {
    
} */

fn motorsRunDirect(motors_sensors: &MotorsSensors) {
    let _ = motors_sensors.lDriveMotor.run_direct(); //SET RUN DIRECT MODE 
    let _ = motors_sensors.rDriveMotor.run_direct(); //SET RUN DIRECT MODE
    let _ = motors_sensors.lToolMotor.run_direct(); //SET RUN DIRECT MODE
    let _ = motors_sensors.rToolMotor.run_direct(); //SET RUN DIRECT MODE

    let _ = motors_sensors.lDriveMotor.set_stop_action("brake"); //SET BRAKE MODE TO PASSIVE ELECTRICAL BRAKE
    let _ = motors_sensors.rDriveMotor.set_stop_action("brake"); //SET BRAKE MODE TO PASSIVE ELECTRICAL BRAKE
    let _ = motors_sensors.lToolMotor.set_stop_action("brake"); //SET BRAKE MODE TO PASSIVE ELECTRICAL BRAKE
    let _ = motors_sensors.rToolMotor.set_stop_action("brake"); //SET BRAKE MODE TO PASSIVE ELECTRICAL BRAKE
}


pub fn motorsStopCoast(motors_sensors: &MotorsSensors) {

    // Stop all actuators
    motors_sensors.lDriveMotor.set_speed_sp(0).expect("lDrive motor write failed");
    motors_sensors.rDriveMotor.set_speed_sp(0).expect("lDrive motor write failed");
    motors_sensors.lToolMotor.set_duty_cycle_sp(0).expect("lDrive motor write failed");
    motors_sensors.rToolMotor.set_duty_cycle_sp(0).expect("lDrive motor write failed");

    let _ = motors_sensors.lDriveMotor.set_stop_action("coast"); //SET BRAKE MODE TO PASSIVE ELECTRICAL BRAKE
    let _ = motors_sensors.rDriveMotor.set_stop_action("coast"); //SET BRAKE MODE TO PASSIVE ELECTRICAL BRAKE
    let _ = motors_sensors.lToolMotor.set_stop_action("coast"); //SET BRAKE MODE TO PASSIVE ELECTRICAL BRAKE
    let _ = motors_sensors.rToolMotor.set_stop_action("coast"); //SET BRAKE MODE TO PASSIVE ELECTRICAL BRAKE

    let _ = motors_sensors.lDriveMotor.stop();
    let _ = motors_sensors.rDriveMotor.stop();
    let _ = motors_sensors.lToolMotor.stop();
    let _ = motors_sensors.rToolMotor.stop();
}

pub fn prepare_motors_sensor(port_definitions: &PortDefinition, speed_p: f32, speed_i: f32, speed_d: f32) -> Result<MotorsSensors, &str> {
    // Try to init all motors and sensors
    // Panics (throws error) if not available

    let lDriveMotor = match LargeMotor::get(port_definitions.lDriveMotorPort){
        Ok(m)  => m,
        Err(e) => return Err("MISSING B"),
    };

    let rDriveMotor = match LargeMotor::get(port_definitions.rDriveMotorPort){
        Ok(m)  => m,
        Err(e) => return Err("MISSING C"),
    };

    let lToolMotor = match MediumMotor::get(port_definitions.lToolMotorPort){
        Ok(m)  => m,
        Err(e) => return Err("MISSING D"),
    };
    let rToolMotor = match MediumMotor::get(port_definitions.rToolMotorPort){
        Ok(m)  => m,
        Err(e) => return Err("MISSING A"),
    };

    let gyroSens = match GyroSensor::get(port_definitions.gyroSensPort){
        Ok(m)  => m,
        Err(e) => return Err("MISSING 1"),
    };

    let colourSens = match ColorSensor::get(port_definitions.colourSensPort){
        Ok(m)  => m,
        Err(e) => return Err("MISSING 4"),
    };

    let motors_sensors = MotorsSensors {
        lDriveMotor: lDriveMotor,
        rDriveMotor: rDriveMotor,
        lToolMotor: lToolMotor,
        rToolMotor: rToolMotor,
        gyroSens: gyroSens,
        colourSens: colourSens,
        button: Button::new()
            .expect("failed to load buttons"),
    };

    /* let mut motors_sensors = MotorsSensors {
    }; */

    info!("Current PID values: P: {}, I: {}, D: {}", motors_sensors.lDriveMotor.get_speed_pid_kp().expect(""), motors_sensors.lDriveMotor.get_speed_pid_ki().expect(""), motors_sensors.lDriveMotor.get_speed_pid_kd().expect(""));
    let _ = motors_sensors.lDriveMotor.set_speed_pid_kp(speed_p);
    let _ = motors_sensors.lDriveMotor.set_speed_pid_ki(speed_i);
    let _ = motors_sensors.lDriveMotor.set_speed_pid_kd(speed_d);
    let _ = motors_sensors.rDriveMotor.set_speed_pid_kp(speed_p);
    let _ = motors_sensors.rDriveMotor.set_speed_pid_ki(speed_i);
    let _ = motors_sensors.rDriveMotor.set_speed_pid_kd(speed_d);
    info!("Current PID values: P: {}, I: {}, D: {}", motors_sensors.lDriveMotor.get_speed_pid_kp().expect(""), motors_sensors.lDriveMotor.get_speed_pid_ki().expect(""), motors_sensors.lDriveMotor.get_speed_pid_kd().expect(""));

    motorsRunDirect(&motors_sensors);
    let _ = motors_sensors.gyroSens.set_mode_gyro_ang(); // ============================== MAYBE this is causing reset of gyro??? =========================================
    let _ = motors_sensors.colourSens.set_mode_col_reflect();

    Ok(motors_sensors)
}

//set speed pid values for drive motors
pub fn setSpeedPID(motors_sensors: &MotorsSensors, speed_p: f32, speed_i: f32, speed_d: f32) {
    // show current pid values
    
}