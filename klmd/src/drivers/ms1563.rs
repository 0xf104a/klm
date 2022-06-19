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
use crate::util::log;
use crate::util::color;

use hidapi::HidApi;
use hidapi::HidDevice;

const TAG: &'static str = "MS1563";
const VENDOR_ID: u16 = 0x1462;
const PRODUCT_ID: u16 = 0x1563;

pub struct MS1563{
    device: hidapi::HidDevice,
}

impl MS1563 {
    fn get_buffer() -> [u8; 64] {
        let mut buffer = [0; 64];
        buffer[0] = 0x02;
        buffer
    }

    fn write_buffer(&self, buffer: &[u8; 64]) -> bool{
        if let Ok(_) = self.device.send_feature_report(buffer){
            log::d(TAG, "Succesfully written buffer.");
            true
        }else{
            log::e(TAG, "Failed writing buffer.");
            false
        }
    }
}

impl driver::Driver for MS1563{
    fn new(api: &hidapi::HidApi) -> Option<MS1563> {
        log::i(TAG, "Opening MS1563 device");
        if let Ok(_device) = api.open(VENDOR_ID, PRODUCT_ID){
            Some(MS1563 {
                device: _device,
            })
        } else {
            log::e(TAG,"Opening device failed. Check that program has right access rights.");
            log::panic(TAG, "Unable to open MS1563 device!");
            None
        }
    }

    //TODO: implement all methods

    fn is_present(api: &hidapi::HidApi) -> bool {
        log::e(TAG, "is_present not implemented");
        true
    }

    fn set_color(&self, color: &color::RGB, _brightness: u8) -> bool{
        let mut brightness = _brightness;
        if brightness > 10 {
            log::w(TAG, &format!("Requested brightnesss is too big: {}", brightness));
            brightness = 10;
        }
        let mut buffer = MS1563::get_buffer();
        buffer[2] = 0x01;
        buffer[4] = brightness;
        buffer[5] = 0x01;
        buffer[6] = color.r;
        buffer[7] = color.g;
        buffer[8] = color.b;
        self.write_buffer(&buffer)
    }

    fn set_breathing(&self, colors: &Vec<color::RGB>, _brightness: u8, _speed: u8) -> bool{
        if colors.len() > 7 {
            log::w(TAG, "Color vector is too large, ignoring request");
            return false;
        }
        let mut brightness = _brightness;
        if brightness > 10 {
            log::w(TAG, &format!("Requested brightnesss is too big: {}", brightness));
            brightness = 10;
        }
        let mut speed = _speed;
        if speed > 2{
            log::w(TAG, &format!("Requested speed is too big: {}", speed));
            speed = 2;
        }
        let mut buffer = MS1563::get_buffer();
        buffer[2] = 0x02;
        buffer[3] = speed;
        buffer[4] = brightness;
        buffer[5] = colors.len() as u8;
        let mut color_ptr = 6;
        for color in colors {
            buffer[color_ptr] = color.r;
            buffer[color_ptr + 1] = color.g;
            buffer[color_ptr + 2] = color.b;
            color_ptr += 3;
        }
        self.write_buffer(&buffer)
    }

    fn set_shift(&self, colors: &Vec<color::RGB>, _brightness: u8, _speed: u8) -> bool{
        if colors.len() > 7 {
            log::w(TAG, "Color vector is too large, ignoring request");
            return false;
        }
        let mut brightness = _brightness;
        if brightness > 10 {
            log::w(TAG, &format!("Requested brightnesss it too big: {}", brightness));
            brightness = 10;
        }
        let mut speed = _speed;
        if speed > 2{
            log::w(TAG, &format!("Requested speed is too big: {}", speed));
            speed = 2;
        }
        let mut buffer = MS1563::get_buffer();
        buffer[2] = 0x05;
        buffer[3] = speed;
        buffer[4] = brightness;
        buffer[5] = colors.len() as u8;
        let mut color_ptr = 6;
        for color in colors {
            buffer[color_ptr] = color.r;
            buffer[color_ptr + 1] = color.g;
            buffer[color_ptr + 2] = color.b;
            color_ptr += 3;
        }
        self.write_buffer(&buffer)
    }

    fn set_power(&self, value: bool) -> bool{
        if !value {
            log::d(TAG, "Powering off keyboard lightning");
            self.write_buffer(&MS1563::get_buffer());
            true
        }else{
            log::e(TAG, "Powering on keyboard lightning is not supported for MS1563");
            false
        }
    }
}
