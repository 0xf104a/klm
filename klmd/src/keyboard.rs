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

const TAG: &'static str = "keyboard";

#[derive(PartialEq)]
pub enum KeyboardState{
    KEYBOARD_OFF,
    KEYBOARD_STEADY,
    KEYBOARD_BREATHING,
    KEYBOARD_COLOR_SHIFT,
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
        }
    }

    pub fn sync(&self){
        if !self.sync {
            log::w("Sync is called, when keyboard syncing is off");
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

    pub reset_colors(&mut self){
        self.colors = vec![];
    }
}
