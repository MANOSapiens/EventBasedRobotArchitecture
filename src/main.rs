// Import crates
extern crate ev3dev_lang_rust;
extern crate image;
extern crate imageproc;
use glob::glob;
use image::Rgb;
use std::path::{Path};
use rusttype::{Font, Scale};

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
    let button = Button::new().expect("buttons cannot be loaded");

    let font_data: &[u8] = include_bytes!("NotoSans-Regular.ttf");
    let font: Font<'static> = Font::try_from_bytes(font_data).expect("cant read font");

    let port_definitions = PortDefinition {
        lDriveMotorPort: MotorPort::OutB,
        rDriveMotorPort: MotorPort::OutC,
        lToolMotorPort: MotorPort::OutD,
        rToolMotorPort: MotorPort::OutA,
        gyroSensPort: SensorPort::In1,
        colourSensPort: SensorPort::In4,
    };

    let mut paths = Vec::new();
    let mut filenames = Vec::new();
    let mut index: usize = 0;

    for entry in glob("instructions/*.json").expect("Failed to read glob pattern") {
        match entry {
            Ok(path_buf) => {
                let path: &Path = path_buf.as_ref();

                // Convert &Path to &str
                let path_str: &str = path.to_str().unwrap();
                paths.push(path_str.to_string());

                let l1: &str = path_str.split("/").nth(1).unwrap();
                let l2: &str = l1.split(".").nth(0).unwrap();
                filenames.push(l2.to_string())
            },
            Err(e) => println!("{:?}", e),
        }
    }
    
    loop {
        button.process();
        if button.is_backspace() {
            break;
        }

        if button.is_enter() {
            startExecution(&*paths.get(index).expect("index out of bounds"), &port_definitions);
        }

        if button.is_right() && index < paths.len()-1{
            index += 1;
            std::thread::sleep(std::time::Duration::from_millis(250));
        }

        if button.is_left() && index > 0{
            index -= 1;
            std::thread::sleep(std::time::Duration::from_millis(250));
        }


        screen.clear();
        imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 0, 20, Scale::uniform(25.0), &font, filenames.get(index).expect("index out of bounds"));
        screen.update();
        
    }

    
}
