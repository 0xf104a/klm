/**
 * This file is part of KLMd project.
 *
 *  Copyright 2022 by Polar <toddot@protonmail.com>
 *
 *  Licensed under GNU General Public License 3.0 or later.
 *  Some rights reserved. See COPYING, AUTHORS.
 *
 * @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
 */

use crate::drivers::driver;
use crate::util::color;
use crate::util::log;

use std::io::Write;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

const TAG: &'static str = "keyboard";
const CACHE_FILENAME: &'static str = "/var/cache/klm/klm.state";

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub enum KeyboardState{
    KEYBOARD_OFF,
    KEYBOARD_STEADY,
    KEYBOARD_BREATHING,
    KEYBOARD_COLOR_SHIFT,
}

//Implements a KeyboardState which can be serialization/desearliazation
impl KeyboardState{
    pub fn from_u8(byte: u8) -> Option<KeyboardState> {
        if byte == 0x0 {
            Some(KeyboardState::KEYBOARD_OFF)
        }else if byte == 0x01 {
            Some(KeyboardState::KEYBOARD_STEADY)
        }else if byte == 0x02 {
            Some(KeyboardState::KEYBOARD_BREATHING)
        }else if byte == 0x03 {
            Some(KeyboardState::KEYBOARD_COLOR_SHIFT)
        }else{
            None
        }
    }

    pub fn to_u8(state: KeyboardState) -> u8 {
        if state == KeyboardState::KEYBOARD_OFF {
            0x0
        } else if state == KeyboardState::KEYBOARD_STEADY {
            0x01
        } else if state == KeyboardState::KEYBOARD_BREATHING {
            0x02
        } else if state == KeyboardState::KEYBOARD_COLOR_SHIFT {
            0x03
        } else {
            todo!("to_u8: unimplemented state");
        }
    }
}
//Implements a controller which stores state of keyboard
//and communicates with driver
pub struct Keyboard{
    driver: Box<dyn driver::Driver>,
    state: KeyboardState,
    colors: Vec<color::RGB>,
    brightness: u8,
    speed: u8,
    syncing: bool,
    power: bool,
}

impl Keyboard {
    pub fn new(_driver: Box<dyn driver::Driver>) -> Keyboard{
        Keyboard{
            driver: _driver,
            state: KeyboardState::KEYBOARD_OFF,
            colors: vec![color::RGB::new(0, 0, 0)],
            brightness: 0,
            speed: 0,
            syncing: false,
            power: false,
        }
    }

    pub fn sync(&self){
        if !self.syncing {
            log::w(TAG, "Sync is called, when keyboard syncing is off");
        }
        if !self.power {
            self.driver.set_power(false);
            return ;
        }
        if self.state == KeyboardState::KEYBOARD_OFF {
            self.driver.set_power(false);
        } else if self.state == KeyboardState::KEYBOARD_STEADY {
            if self.colors.len() == 0{
                log::panic(TAG, "Can not synchronize state: empty colors array!");
            }
            if self.brightness == 0{
                log::w(TAG, "Brightness is 0");
            }
            self.driver.set_color(&self.colors[0], self.brightness);
        } else if self.state == KeyboardState::KEYBOARD_BREATHING {
            if self.colors.len() == 0{
                log::panic(TAG, "Can not synchronize state: empty colors array!");
            }
            if self.brightness == 0{
                log::w(TAG, "Brightness is 0");
            }
            self.driver.set_breathing(&self.colors, self.brightness, self.speed);
        } else if self.state == KeyboardState::KEYBOARD_COLOR_SHIFT {
            if self.colors.len() == 0{
                log::panic(TAG, "Can not synchronize state: empty colors array!");
            }
            if self.brightness == 0{
                log::w(TAG, "Brightness is 0");
            }
            self.driver.set_shift(&self.colors, self.brightness, self.speed);
        }
    }

    pub fn lock_sync(&mut self){
        self.syncing = false;
    }

    pub fn unlock_sync(&mut self){
        self.syncing = true;
    }

    pub fn set_state(&mut self, state: KeyboardState){
        self.state = state;
        if self.syncing {
            self.sync();
        }
    }

    pub fn set_color(&mut self, color: color::RGB){
        self.colors = vec![color];
        if self.syncing {
            self.sync();
        }
    }

    pub fn add_color(&mut self, color: color::RGB){
        self.colors.push(color);
        if self.syncing {
            self.sync();
        }
    }

        pub fn set_brightness(&mut self, brightness: u8){
        self.brightness = brightness;
        if self.syncing {
            self.sync();
        }
    }

    pub fn set_speed(&mut self, speed: u8){
        self.speed = speed;
        if self.syncing {
            self.sync();
        }
    }

    pub fn reset_colors(&mut self){
        self.colors = vec![];
    }

    pub fn set_power(&mut self, power: bool){
        self.power = power;
    }

    pub fn save_state(&self) -> bool {
        //Prepare buffer
        let mut buffer = Vec::<u8>::new();
        buffer.push(self.brightness);
        buffer.push(self.speed);
        buffer.push(KeyboardState::to_u8(self.state));
        if self.power{
            buffer.push(0x01);
        } else {
            buffer.push(0x00);
        }
        if(self.colors.len() > 255){
            log::panic(TAG, "Too many colors. Maybe a bug?");
        }
        buffer.push(self.colors.len().try_into().unwrap());
        for color in &self.colors {
            buffer.push(color.r);
            buffer.push(color.g);
            buffer.push(color.b);
        }
        //Write to buffer to file
        let mut file = File::create(CACHE_FILENAME).expect("Unable to create file");
        file.write_all(&buffer).expect("Unable to write buffer");
        true
    }


    fn load_state(&mut self) -> bool {
        let mut file = File::open(CACHE_FILENAME).expect("Unable to open file");
        let mut state_buffer = [0u8; 1];
        let mut color_buffer = [0u8; 3];
        //Read Brightness
        file.read_exact(&mut state_buffer);
        self.brightness = state_buffer[0];
        //Read speed
        file.read_exact(&mut state_buffer);
        self.speed = state_buffer[0];
        //Read state
        file.read_exact(&mut state_buffer);
        self.state = KeyboardState::from_u8(state_buffer[0]).expect("Bad state specifier");
        //Read power
        file.read_exact(&mut state_buffer);
        let power_byte = state_buffer[0];
        if power_byte == 0x0 {
            self.power = false;
        } else {
            self.power = true;
        }
        //Read number of colors
        file.read_exact(&mut state_buffer);
        let n = state_buffer[0];
        self.colors = Vec::<color::RGB>::new();
        for _ in 0..n {
            file.read_exact(&mut color_buffer);
            self.colors.push(color::RGB::new(color_buffer[0], color_buffer[1], color_buffer[2]));
        }
        true
    }

    pub fn load_state_if_exists(&mut self) -> bool{
        if Path::new(CACHE_FILENAME).exists() {
            log::i(TAG, &format!("Loading previous keyboard state from {}", CACHE_FILENAME));
            self.load_state()
        } else {
            false
        }
    }

}
