// Import crates
extern crate ev3dev_lang_rust;
extern crate image;
extern crate imageproc;
use glob::glob;
use image::Rgb;

use ev3dev_lang_rust::motors::{MotorPort};
use ev3dev_lang_rust::sensors::{SensorPort};
use ev3dev_lang_rust::Screen;
use ev3dev_lang_rust::Button;

// Local modules
pub mod Events;
mod Ports;
use crate::Ports::PortDefinition;

mod Logger;
use crate::Logger::init_logger;

// Function Definition
mod Actuators;
mod Check;
mod PID;
mod ProcessLoop;
mod ReadInstructions;
mod ReadSensors;
mod RunEvents;
mod SpawnTerminateEvents;
mod StartExecution;
pub mod consts;

use crate::consts::*;
use crate::StartExecution::startExecution;

fn main() {
    // Initialize logger
    let logger_config_file = String::from("log/log4rs.yaml");
    init_logger(logger_config_file);

    let mut screen = Screen::new().unwrap();
    let button = Button::new().unwrap();

    let port_definitions = PortDefinition {
        lDriveMotorPort: MotorPort::OutB,
        rDriveMotorPort: MotorPort::OutC,
        lToolMotorPort: MotorPort::OutD,
        rToolMotorPort: MotorPort::OutA,
        gyroSensPort: SensorPort::In1,
        colourSensPort: SensorPort::In4,
    };

    let mut paths = Vec::new();
    let mut index: usize = 0;

    for entry in glob("instructions/*.json").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => paths.push(path),
            Err(e) => println!("{:?}", e),
        }
    }

    loop {
        if button.is_backspace() {
            break;
        }

        if button.is_enter() {
            startExecution(paths.get(index).expect("index out of bounds"), port_definitions);
        }

        if button.is_right() && index < paths.len(){
            index += 1;
        }

        if button.is_left() && index > 0{
            index -= 1;
        }


        imageproc::drawing::draw_filled_circle_mut(&mut screen.image, (100, 50), 40, Rgb([0, 0, 255]));
        imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 255]), 50, 40 , 5, font, paths.get(index).expect("index out of bounds").display());
    }

    
}
