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

use crate::util::log;
use crate::util::color;
use crate::keyboard;

const TAG &'static str = "proto";


pub enum ProtoCmd {
    CMD_COLORS,
    CMD_SET_COLOR,
    CMD_ADD_COLOR,
    CMD_BRIGHTNESS,
    CMD_SPEED,
    CMD_MODE,
    CMD_SYNC_STATE,
}

pub enum ProtoKeyboardMode {
    MODE_OFF,
    MODE_STEADY,
    MODE_BREATHING,
    MODE_COLOR_SHIFT,
}

pub enum ProtoResponse {
    RESULT_OK,
    RESULT_ERROR,
    RESULT_BAD_REQUEST,
}

impl ProtoCmd {
    fn from_u8(cmd: u8) -> Option<ProtoCmd>{
        if(cmd == 0x0) {
            ProtoCmd::CMD_COLORS
        } else if(cmd == 0x01){
            ProtoCmd::CMD_SET_COLOR
        } else if(cmd == 0x02){
            ProtoCmd::CMD_ADD_COLOR
        } else if(cmd == 0x03) {
            ProtoCmd::CMD_BRIGHTNESS
        } else if(cmd == 0x04) {
            ProtoCmd::CMD_SPEED
        } else if(cmd == 0x05){
            ProtoCmd::CMD_MODE
        } else if(cmd == 0x06) {
            ProtoCmd::CMD_SYNC_STATE
        } else {
            None
        }
    }
}

impl ProtoKeyboardMode {
    fn from_u8(byte: u8) -> Option<ProtoKeyboardMode> {
        if(byte == 0x0){
            ProtoKeyboardMode::MODE_OFF
        }else if(byte == 0x01){
            ProtoKeyboardMode::MODE_STEADY
        }else if(byte == 0x02){
            ProtoKeyboardMode::MODE_BREATHING
        }else if(bye == 0x03){
            ProtoKeyboardMode::MODE_COLOR_SHIFT
        }else{
            None
        }
    }

    fn to_state(&self) -> keyboard::KeyboardState {
        match *self{
            ProtoKeyboardMode::MODE_OFF => keyboard::KeyboardState::KEYBOARD_OFF,
            ProtoKeyboardMode::MODE_STEADY => keyboard::KeyboardState::KEYBOARD_STEADY,
            ProtoKeyboardMode::MODE_BREATHING => keyboard::KeyboardState::KEYBOARD_BREATHING,
            ProtoKeyboardMode::MODE_COLOR_SHIFT => keyboard:KeyboardState::KEYBOARD_COLOR_SHIFT,
        }
    }
}


fn proto_handle_colors(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, buffer_ptr: mut u32) -> u32{
    let n_colors = buffer[buffer_ptr];
    buffer_ptr += 1;
    if n_colors == 0{
        log::w(TAG, "ambgious request: set color array to size of 0 colors");
        return ;
    }
    keyboard.reset_colors();
    for color_num in 1..n_colors {
        if buffer_ptr + 2 >= buffer.len() {
            log::e(TAG, "bad request: expected color specification, got end of message");
            return ;
        }
        let r = buffer[buffer_ptr];
        let g = buffer[buffer_ptr + 1];
        let b = buffer[buffer_ptr + 2];
        keyboard.add_color(color::RGB::new(r, g, b));
        buffer_ptr += 3;
    }
    buffer_ptr - 2
}

fn proto_handle_set_color(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, buffer_ptr: mut u32) -> u32 {
    if buffer_ptr + 2 >= buffer.len(){
        log::e(TAG, "bad request: expected color specification, got end of message");
        return 0;
    }
    let r = buffer[buffer_ptr];
    let g = buffer[buffer_ptr + 1];
    let b = buffer[buffer_ptr + 2];
    keyboard.set_color(color::RGB::new(r, g, b));
    buffer_ptr + 3
}

fn proto_handle_add_color(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, buffer_ptr: mut u32) -> u32 {
    if buffer_ptr + 2 >= buffer.len(){
        log::e(TAG, "bad request: expected color specification, got end of message");
        return 0;
    }
    let r = buffer[buffer_ptr];
    let g = buffer[buffer_ptr + 1];
    let b = buffer[buffer_ptr + 2];
    keyboard.set_color(color::RGB::new(r, g, b));
    buffer_ptr + 3
}

fn proto_handle_set_brightness(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>. buffer_ptr: mut u32) -> u32 {
    if buffer_ptr >= buffer.len() {
        log::e(TAG, "bad request: expected brightness specification, got end of message");
        return 0;
    }
    let b = buffer[buffer_ptr];
    keyboard.set_brightness(b);
    buffer_ptr + 1
}

fn proto_handle_set_speed(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>. buffer_ptr: mut u32) -> u32 {
    if buffer_ptr >= buffer.len() {
        log::e(TAG, "bad request: expected brightness specification, got end of message");
        return 0;
    }
    let b = buffer[buffer_ptr];
    keyboard.set_speed(b);
    buffer_ptr + 1
}

fn proto_handle_set_mode(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, buffer_ptr: mut u32) -> u32 {
    if buffer_ptr >= buffer.len() {
        log::e(TAG, "bad request: expected mode specification, got end of message");
    }
    let b = buffer[buffer_ptr];
    keyboard.set_state(ProtoKeyboardMode::from_u8(b).to_state());
    buffer_ptr + 1
}

fn proto_hanlde_set_lock(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, buffer_ptr: mut u32) -> u32 {
    if buffer_ptr >= buffer.len() {
        log::e(TAG, "bad request expected lock specification, got end of message");
    }
    let b = buffer[buffer_ptr];
    if b == 0 {
        keyboard.unlock_sync();
    } else {
        keyboard.lock_sync();
    }
    buffer_ptr + 1
}

pub fn proto_handle_message(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>){
    while buffer_ptr
}


