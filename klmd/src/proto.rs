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

const TAG: &'static str = "proto";

#[derive(PartialEq)]
pub enum ProtoCmd {
    CMD_COLORS,
    CMD_SET_COLOR,
    CMD_ADD_COLOR,
    CMD_BRIGHTNESS,
    CMD_SPEED,
    CMD_MODE,
    CMD_SYNC_STATE,
    CMD_POWER,
}

#[derive(PartialEq)]
pub enum ProtoKeyboardMode {
    MODE_OFF,
    MODE_STEADY,
    MODE_BREATHING,
    MODE_COLOR_SHIFT,
}

#[derive(PartialEq)]
pub enum ProtoResponse {
    RESULT_OK,
    RESULT_ERROR,
    RESULT_BAD_REQUEST,
}

impl ProtoCmd {
    pub fn from_u8(cmd: u8) -> Option<ProtoCmd>{
        if cmd == 0x0 {
            Some(ProtoCmd::CMD_COLORS)
        } else if cmd == 0x01 {
            Some(ProtoCmd::CMD_SET_COLOR)
        } else if cmd == 0x02 {
            Some(ProtoCmd::CMD_ADD_COLOR)
        } else if cmd == 0x03 {
            Some(ProtoCmd::CMD_BRIGHTNESS)
        } else if cmd == 0x04 {
            Some(ProtoCmd::CMD_SPEED)
        } else if cmd == 0x05 {
            Some(ProtoCmd::CMD_MODE)
        } else if cmd == 0x06 {
            Some(ProtoCmd::CMD_SYNC_STATE)
        } else if cmd == 0x07{
            Some(ProtoCmd::CMD_POWER)
        } else {
            None
        }
    }
}

impl ProtoKeyboardMode {
    pub fn from_u8(byte: u8) -> Option<ProtoKeyboardMode> {
        if byte == 0x0 {
            Some(ProtoKeyboardMode::MODE_OFF)
        }else if byte == 0x01 {
            Some(ProtoKeyboardMode::MODE_STEADY)
        }else if byte == 0x02 {
            Some(ProtoKeyboardMode::MODE_BREATHING)
        }else if byte == 0x03 {
            Some(ProtoKeyboardMode::MODE_COLOR_SHIFT)
        }else{
            None
        }
    }

    pub fn to_state(&self) -> keyboard::KeyboardState {
        match *self{
            ProtoKeyboardMode::MODE_OFF => keyboard::KeyboardState::KEYBOARD_OFF,
            ProtoKeyboardMode::MODE_STEADY => keyboard::KeyboardState::KEYBOARD_STEADY,
            ProtoKeyboardMode::MODE_BREATHING => keyboard::KeyboardState::KEYBOARD_BREATHING,
            ProtoKeyboardMode::MODE_COLOR_SHIFT => keyboard::KeyboardState::KEYBOARD_COLOR_SHIFT,
        }
    }
}

impl ProtoResponse {
    pub fn to_u8(&self) -> u8 {
        match *self {
            ProtoResponse::RESULT_OK => 0x0,
            ProtoResponse::RESULT_ERROR => 0x1,
            ProtoResponse::RESULT_BAD_REQUEST => 0x2,
        }
    }
}

fn proto_handle_colors(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, mut buffer_ptr: usize) -> usize{
    let n_colors = buffer[buffer_ptr];
    buffer_ptr += 1;
    if n_colors == 0{
        log::w(TAG, "ambgious request: set color array to size of 0 colors");
        return 0;
    }
    keyboard.reset_colors();
    for color_num in 1..n_colors {
        if buffer_ptr + 2 >= buffer.len() {
            log::e(TAG, "bad request: expected color specification, got end of message");
            return 0;
        }
        let r = buffer[buffer_ptr];
        let g = buffer[buffer_ptr + 1];
        let b = buffer[buffer_ptr + 2];
        keyboard.add_color(color::RGB::new(r, g, b));
        buffer_ptr += 3;
    }
    buffer_ptr - 2
}

fn proto_handle_set_color(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, mut buffer_ptr: usize) -> usize {
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

fn proto_handle_add_color(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, mut buffer_ptr: usize) -> usize {
    if buffer_ptr + 2 >= buffer.len(){
        log::e(TAG, "bad request: expected color specification, got end of message");
        return 0;
    }
    let r = buffer[buffer_ptr];
    let g = buffer[buffer_ptr + 1];
    let b = buffer[buffer_ptr + 2];
    keyboard.add_color(color::RGB::new(r, g, b));
    buffer_ptr + 3
}

fn proto_handle_set_brightness(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, mut buffer_ptr:  usize) -> usize {
    if buffer_ptr >= buffer.len() {
        log::e(TAG, "bad request: expected brightness specification, got end of message");
        return 0;
    }
    let b = buffer[buffer_ptr];
    keyboard.set_brightness(b);
    buffer_ptr + 1
}

fn proto_handle_set_speed(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, mut buffer_ptr: usize) -> usize {
    if buffer_ptr >= buffer.len() {
        log::e(TAG, "bad request: expected brightness specification, got end of message");
        return 0;
    }
    let b = buffer[buffer_ptr];
    keyboard.set_speed(b);
    buffer_ptr + 1
}

fn proto_handle_set_mode(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, mut buffer_ptr: usize) -> usize {
    if buffer_ptr >= buffer.len() {
        log::e(TAG, "bad request: expected mode specification, got end of message");
        return 0;
    }
    let b = buffer[buffer_ptr];
    log::d(TAG, &format!("set_mode: {}", b));
    if let Some(mode) = ProtoKeyboardMode::from_u8(b) {
        keyboard.set_state(mode.to_state());
    } else {
        log::e(TAG, &format!("bad request: bad mode specifier {} at {}", b, buffer_ptr));
        return 0;
    }
    buffer_ptr + 1
}

fn proto_handle_set_lock(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, mut buffer_ptr: usize) -> usize {
    if buffer_ptr >= buffer.len() {
        log::e(TAG, "bad request: expected lock specification, got end of message");
    }
    let b = buffer[buffer_ptr];
    if b == 0 {
        keyboard.unlock_sync();
    } else {
        keyboard.lock_sync();
    }
    buffer_ptr + 1
}

fn proto_handle_set_power(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, mut buffer_ptr: usize) -> usize {
    if buffer_ptr >= buffer.len() {
        log::e(TAG, "bad request: expected power specification, got end of message");
    }
    let b = buffer[buffer_ptr];
    if b == 0 {
       keyboard.set_power(false);
    } else {
       keyboard.set_power(true);
    }
    buffer_ptr + 1
}

pub fn proto_handle_message(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>) -> ProtoResponse{
    let mut buffer_ptr = 0;
    if(buffer.len() == 0){
        log::e(TAG, "bad reqeust: empty buffer. This is a bug: must be handled earlier.");
        return ProtoResponse::RESULT_ERROR;
    }
    keyboard.lock_sync();
    //log::d(TAG, &format!("buffer={}", buffer));
    while buffer_ptr < buffer.len() {
        let cmd_byte = buffer[buffer_ptr];
        buffer_ptr += 1;
        let cmd_wrapped = ProtoCmd::from_u8(cmd_byte);
        if cmd_wrapped == None {
            log::e(TAG, &format!("bad request: unknown command {} at pos {}", cmd_byte, buffer_ptr - 1));
            return ProtoResponse::RESULT_BAD_REQUEST;
        }
        let cmd = cmd_wrapped.unwrap();
        log::d(TAG, &format!("cmd={}", cmd_byte));
        if cmd == ProtoCmd::CMD_COLORS {
            buffer_ptr = proto_handle_colors(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CMD_SET_COLOR {
            buffer_ptr = proto_handle_set_color(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CMD_ADD_COLOR {
            buffer_ptr = proto_handle_add_color(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CMD_BRIGHTNESS {
            buffer_ptr = proto_handle_set_brightness(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CMD_SPEED {
            buffer_ptr = proto_handle_set_speed(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CMD_MODE {
            buffer_ptr = proto_handle_set_mode(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CMD_SYNC_STATE {
            buffer_ptr = proto_handle_set_lock(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CMD_POWER {
            buffer_ptr = proto_handle_set_power(keyboard, buffer, buffer_ptr);
        }
        if(buffer_ptr == 0) {
            log::e(TAG, "proto_handle_message: parsing message failed.");
            return ProtoResponse::RESULT_BAD_REQUEST;
        }
    }
    keyboard.save_state();
    keyboard.unlock_sync();
    keyboard.sync();
    ProtoResponse::RESULT_OK
}


