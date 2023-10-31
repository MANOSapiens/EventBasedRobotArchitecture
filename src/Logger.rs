
use std::error::Error;
use std::io::Write;


use crate::ProcessLoop::SensorActuatorValues;


pub fn init_logger(log_file: String) {
    log4rs::init_file(log_file, Default::default()).unwrap();
}

pub fn logHeaderCSV<W: Write>(wtr: &mut csv::Writer<W>) -> Result<(), Box<dyn Error>> {

    wtr.write_record([
        "currentTime",
        "lDriveMotorEnc",
        "rDriveMotorEnc",
        "lToolMotorEnc",
        "rToolMotorEnc",
        "gyroAngValue",
        "lDriveMotorPow",
        "rDriveMotorPow",
        "lToolMotorPow",
        "rToolMotorPow",
        "lDriveMotorCor",
        "rDriveMotorCor",
        "lToolMotorCor",
        "rToolMotorCor",
        "lDriveMotorSpeed",
        "rDriveMotorSpeed",
        ])?;

    wtr.flush()?;
    Ok(())
}


pub fn logCSV<W: Write>(wtr: &mut csv::Writer<W>, sensor_act_values: &mut SensorActuatorValues) -> Result<(), Box<dyn Error>> {

    wtr.serialize(
        (
            sensor_act_values.currentTime, 
            sensor_act_values.lDriveMotorEnc, 
            sensor_act_values.rDriveMotorEnc, 
            sensor_act_values.lToolMotorEnc, 
            sensor_act_values.rToolMotorEnc,
            sensor_act_values.gyroAngValue,
            sensor_act_values.lDriveMotorPow,
            sensor_act_values.rDriveMotorPow,
            sensor_act_values.lToolMotorPow,
            sensor_act_values.rToolMotorPow,
            sensor_act_values.lDriveMotorCor,
            sensor_act_values.rDriveMotorCor,
            sensor_act_values.lToolMotorCor,
            sensor_act_values.rToolMotorCor,
            sensor_act_values.lDriveMotorSpeed,
            sensor_act_values.rDriveMotorSpeed,
        ))?;

    wtr.flush()?;
    Ok(())
}