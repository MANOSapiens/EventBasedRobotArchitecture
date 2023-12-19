// Import crates
extern crate ev3dev_lang_rust;
extern crate image;
extern crate imageproc;

use image::Rgb;
use std::path::{Path};
use rusttype::{Font, Scale};

use ev3dev_lang_rust::Screen;
use ev3dev_lang_rust::Button;
use ev3dev_lang_rust::motors::{MediumMotor, LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{SensorPort, ColorSensor, GyroSensor};


pub struct PortDefinition {
    pub lDriveMotorPort: MotorPort,
    pub rDriveMotorPort: MotorPort,
    pub lToolMotorPort: MotorPort,
    pub rToolMotorPort: MotorPort,
}


fn main() {
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
    };

    let lDriveMotor = LargeMotor::get(port_definitions.lDriveMotorPort).expect("failed to load lDriveMotor");
    let rDriveMotor = LargeMotor::get(port_definitions.rDriveMotorPort).expect("failed to load rDriveMotor");
    let lToolMotor = MediumMotor::get(port_definitions.lToolMotorPort).expect("failed to load lToolMotor");
    let rToolMotor = MediumMotor::get(port_definitions.rToolMotorPort).expect("failed to load rToolMotor");
    let gyroSens = GyroSensor::get(port_definitions.gyroSensPort).expect("failed to load gyroSens");
    gyroSens.set_mode_gyro_ang().expect("failed to set gyroSens to gyro ang mode");

    let _ = lToolMotor.run_direct(); //SET RUN DIRECT MODE
    let _ = rToolMotor.run_direct(); //SET RUN DIRECT MODE
    
    let mut screenSelection: i8 = 0;
    let mut lDriveMotorEncoder: i32 = 0;
    let mut rDriveMotorEncoder: i32 = 0;
    let mut lToolMotorEncoder: i32 = 0;
    let mut rToolMotorEncoder: i32 = 0;
    let mut gyroSensValue: i32 = 0;

    // tool motor power for every motor
    let mut lToolMotorPower: i32 = 0;
    let mut rToolMotorPower: i32 = 0;
    let mut toolMotorPower: i32 = 50;


    loop {
        button.process();

        // switch between 3 screenSelections by pressing the enter button
        if button.is_enter() {
            screenSelection += 1;
            if screenSelection > 2 {
                screenSelection = 0;
            }
            // reset all encoders to zero
            lDriveMotor.set_position(0).expect("failed to reset lDriveMotor encoder");
            rDriveMotor.set_position(0).expect("failed to reset rDriveMotor encoder");
            lToolMotor.set_position(0).expect("failed to reset lToolMotor encoder");
            rToolMotor.set_position(0).expect("failed to reset rToolMotor encoder");

            std::thread::sleep(std::time::Duration::from_millis(150));
        }
        
        // switch case for the 3 screenSelections
        // reset both power to zero
        lToolMotorPower = 0;
        rToolMotorPower = 0;
        

        // read encoders of all motors
        lDriveMotorEncoder = lDriveMotor.get_position().expect("failed to read lDriveMotor encoder");
        rDriveMotorEncoder = rDriveMotor.get_position().expect("failed to read rDriveMotor encoder");
        lToolMotorEncoder = lToolMotor.get_position().expect("failed to read lToolMotor encoder");
        rToolMotorEncoder = rToolMotor.get_position().expect("failed to read rToolMotor encoder");

        match screenSelection {
            0 => {

                screen.clear();
                imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 30, 0, Scale::uniform(25.0), &font, "TOOLMOTOR");
                
                // draw the power of the tool motors

                imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 0, 15, Scale::uniform(50.0), &font, &lToolMotorEncoder.to_string());
                imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 100, 15, Scale::uniform(50.0), &font, &rToolMotorEncoder.to_string());
                
                //imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 20, 80, Scale::uniform(15.0), &font, "LEFT(CW)");
                //imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 20, 100, Scale::uniform(15.0), &font, "RIGHT(CCW)");

                //imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 115, 80, Scale::uniform(15.0), &font, "UP(CW)");
                //imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 115, 100, Scale::uniform(15.0), &font, "DOWN(CCW)");
                
                screen.update();

                // if button pressed assign toolMotorPower to the power of the tool motors
                if button.is_up() {
                    rToolMotorPower = toolMotorPower;
                }
                if button.is_down() {
                    rToolMotorPower = -toolMotorPower;
                }
                if button.is_left() {
                    lToolMotorPower = -toolMotorPower;
                }
                if button.is_right() {
                    lToolMotorPower = toolMotorPower;
                }

                // write power to tool motors
                lToolMotor.set_duty_cycle_sp(lToolMotorPower).expect("lToolMotor write failed");
                rToolMotor.set_duty_cycle_sp(rToolMotorPower).expect("rToolMotor write failed");
            },
            1 => {
                // button up increases the power of the tool motors up to a maximum of 1 in 0.1 steps
                if button.is_up() {
                    toolMotorPower += 10;
                    if toolMotorPower > 100 {
                        toolMotorPower = 100;
                    }
                    //timer
                    std::thread::sleep(std::time::Duration::from_millis(150));
                }

                // button down decreases the power of the tool motors down to a minimum of 0 in 0.1 steps
                if button.is_down() {
                    toolMotorPower -= 10;
                    if toolMotorPower < 0 {
                        toolMotorPower = 0;
                    }
                    //timer
                    std::thread::sleep(std::time::Duration::from_millis(150));
                }

                screen.clear();
                imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 25, 0, Scale::uniform(25.0), &font, "SELECT POWER");
                imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 50, 20, Scale::uniform(100.0), &font, &toolMotorPower.to_string());
                screen.update();
            },
            2 => {
                gyroSensValue = gyroSens.get_angle().expect("failed to read gyroSens");
                screen.clear();
                imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 28, 0, Scale::uniform(25.0), &font, "DRIVEMOTOR");
                
                // draw the power of the tool motors

                imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 0, 15, Scale::uniform(50.0), &font, &lDriveMotorEncoder.to_string());
                imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 100, 15, Scale::uniform(50.0), &font, &rDriveMotorEncoder.to_string());
                imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 50, 50, Scale::uniform(50.0), &font, &gyroSensValue.to_string());
                
                //imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 20, 80, Scale::uniform(15.0), &font, "LEFT(CW)");
                //imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 20, 100, Scale::uniform(15.0), &font, "RIGHT(CCW)");

                //imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 115, 80, Scale::uniform(15.0), &font, "UP(CW)");
                //imageproc::drawing::draw_text_mut(&mut screen.image, Rgb([0, 0, 0]), 115, 100, Scale::uniform(15.0), &font, "DOWN(CCW)");
                
                screen.update();
            },
            _ => {
            }
        }

        

    }

    

    
}
