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

use std::os::unix::net::UnixListener;

use crate::drivers::driver;
use crate::drivers::ms1563;
use crate::drivers::driver::Driver;
use crate::util::log;
use crate::util::color::RGB;

const TAG: &'static str = "main";

fn main(){
    log::i(TAG, format!("klmd versiom {} starting.", VERSION));
    log::w(TAG. "This version is early alpha and is not intended to be used in product mode. Many features are not yet implemnted.");

    if let Ok(api) = hidapi::HidApi::new(){
        log::i(TAG, "Succesfully initialized HID API.")
    }else{
        log::e(TAG, "Failed to open HID API.");
        log::e(TAG, "Check that program has permissions to access HID devices.");
        log::panic(TAG, "No HID API availiable!");
    }

    if !ms1563::MS1563::is_present(api){
        log::e(TAG, "This program supports only MS1563 keyboards.");
        log::panic(TAG, "No compatiable keyboard found!");
    }

    let driver = Box(ms1563::MS1563::new(api));
    let mut keyboard = keyboard::new(driver);
    //Listen for communication
    let listener = UnixListener::bind("/var/run/klm.sock");



}
