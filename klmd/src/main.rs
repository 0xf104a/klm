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

extern crate hidapi;

mod drivers;
mod util;
mod keyboard;
mod proto;
mod listener;


use std::os::unix::net::UnixListener;

use crate::drivers::driver;
use crate::drivers::ms1563;
use crate::drivers::driver::Driver;
use crate::util::log;
use crate::util::color::RGB;

const TAG: &'static str = "main";
const VERSION: &'static str = "0.1.1"; //TODO: synchronize with cargo?

fn main(){
    log::i(TAG, &format!("klmd version {} starting.", VERSION));
    log::w(TAG, "This version is early alpha and is not intended to be used in product mode. Many features are not yet implemnted.");


    let api = match hidapi::HidApi::new() {
        Ok(api) => Some(api),
        Err(e) => {
            log::panic(TAG, &format!("Can not initialize HID API: {}", e));
            None
        },
    }.unwrap();

    //TODO: here the dynamic loading of drivers should happen
    if !ms1563::MS1563::is_present(&api){
        log::e(TAG, "This program supports only MS1563 keyboards.");
        log::panic(TAG, "No compatiable keyboard found!");
    }

    let driver = Box::new(ms1563::MS1563::new(&api).unwrap());
    let mut keyboard = keyboard::Keyboard::new(driver);
    listener::listen(&mut keyboard);

}
