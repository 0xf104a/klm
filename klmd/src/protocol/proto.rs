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
use crate::protocol::response::{ProtoResponse, ProtoResponseState};

const TAG: &'static str = "proto";

#[derive(PartialEq)]
pub enum ProtoCmd {
    CmdColors,
    CmdSetColor,
    CmdAddColor,
    CmdBrightness,
    CmdSpeed,
    CmdMode,
    CmdSyncState,
    CmdPower,
    CmdToggle,
    CmdReqModesAvail,
}

#[derive(PartialEq)]
pub enum ProtoKeyboardMode {
    ModeOff,
    ModeSteady,
    ModeBreathing,
    ModeColorShift,
}

impl ProtoCmd {
    pub fn from_u8(cmd: u8) -> Option<ProtoCmd> {
        if cmd == 0x0 {
            Some(ProtoCmd::CmdColors)
        } else if cmd == 0x01 {
            Some(ProtoCmd::CmdSetColor)
        } else if cmd == 0x02 {
            Some(ProtoCmd::CmdAddColor)
        } else if cmd == 0x03 {
            Some(ProtoCmd::CmdBrightness)
        } else if cmd == 0x04 {
            Some(ProtoCmd::CmdSpeed)
        } else if cmd == 0x05 {
            Some(ProtoCmd::CmdMode)
        } else if cmd == 0x06 {
            Some(ProtoCmd::CmdSyncState)
        } else if cmd == 0x07 {
            Some(ProtoCmd::CmdPower)
        } else if cmd == 0x08 {
            Some(ProtoCmd::CmdToggle)
        } else if cmd == 0x09 {
            Some(ProtoCmd::CmdReqModesAvail)
        } else {
            None
        }
    }
}

impl ProtoKeyboardMode {
    pub fn from_u8(byte: u8) -> Option<ProtoKeyboardMode> {
        if byte == 0x0 {
            Some(ProtoKeyboardMode::ModeOff)
        } else if byte == 0x01 {
            Some(ProtoKeyboardMode::ModeSteady)
        } else if byte == 0x02 {
            Some(ProtoKeyboardMode::ModeBreathing)
        } else if byte == 0x03 {
            Some(ProtoKeyboardMode::ModeColorShift)
        } else {
            None
        }
    }

    pub fn to_state(&self) -> keyboard::KeyboardState {
        match *self {
            ProtoKeyboardMode::ModeOff => keyboard::KeyboardState::KeyboardOff,
            ProtoKeyboardMode::ModeSteady => keyboard::KeyboardState::KeyboardSteady,
            ProtoKeyboardMode::ModeBreathing => keyboard::KeyboardState::KeyboardBreathing,
            ProtoKeyboardMode::ModeColorShift => keyboard::KeyboardState::KeyboardColorShift,
        }
    }
}

fn proto_handle_colors(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, mut buffer_ptr: usize) -> usize {
    let n_colors = buffer[buffer_ptr];
    buffer_ptr += 1;
    if n_colors == 0 {
        log::w(TAG, "ambgious request: set color array to size of 0 colors");
        return 0;
    }
    keyboard.reset_colors();
    for _color_num in 1..n_colors {
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

fn proto_handle_set_color(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, buffer_ptr: usize) -> usize {
    if buffer_ptr + 2 >= buffer.len() {
        log::e(TAG, "bad request: expected color specification, got end of message");
        return 0;
    }
    let r = buffer[buffer_ptr];
    let g = buffer[buffer_ptr + 1];
    let b = buffer[buffer_ptr + 2];
    keyboard.set_color(color::RGB::new(r, g, b));
    buffer_ptr + 3
}

fn proto_handle_add_color(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, buffer_ptr: usize) -> usize {
    if buffer_ptr + 2 >= buffer.len() {
        log::e(TAG, "bad request: expected color specification, got end of message");
        return 0;
    }
    let r = buffer[buffer_ptr];
    let g = buffer[buffer_ptr + 1];
    let b = buffer[buffer_ptr + 2];
    keyboard.add_color(color::RGB::new(r, g, b));
    buffer_ptr + 3
}

fn proto_handle_set_brightness(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, buffer_ptr: usize) -> usize {
    if buffer_ptr >= buffer.len() {
        log::e(TAG, "bad request: expected brightness specification, got end of message");
        return 0;
    }
    let b = buffer[buffer_ptr];
    keyboard.set_brightness(b);
    buffer_ptr + 1
}

fn proto_handle_set_speed(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, buffer_ptr: usize) -> usize {
    if buffer_ptr >= buffer.len() {
        log::e(TAG, "bad request: expected brightness specification, got end of message");
        return 0;
    }
    let b = buffer[buffer_ptr];
    keyboard.set_speed(b);
    buffer_ptr + 1
}

fn proto_handle_set_mode(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, buffer_ptr: usize) -> usize {
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

fn proto_handle_set_lock(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, buffer_ptr: usize) -> usize {
    if buffer_ptr >= buffer.len() {
        log::e(TAG, "bad request: expected lock specification, got end of message");
        return 0;
    }
    let b = buffer[buffer_ptr];
    if b == 0 {
        keyboard.unlock_sync();
    } else {
        keyboard.lock_sync();
    }
    buffer_ptr + 1
}


fn proto_handle_set_power(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>, buffer_ptr: usize) -> usize {
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


fn proto_handle_toggle_power(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>,
                             buffer_ptr: usize) -> usize {
    if buffer_ptr > buffer.len() {
        log::e(TAG, "bad request: buffer_ptr is out of range");
        return 0;
    }
    keyboard.toggle_power();
    buffer_ptr
}

fn proto_handle_request_modes(keyboard: &keyboard::Keyboard, buffer: &Vec<u8>,
                              buffer_ptr: usize, response: &mut ProtoResponse) -> usize {
    if buffer_ptr > buffer.len() {
        log::e(TAG, "bad request: buffer_ptr is out of range");
        return 0;
    }
    let modes = keyboard.get_color_modes();
    response.add_response(Box::new(modes));
    buffer_ptr
}

pub fn proto_handle_message(keyboard: &mut keyboard::Keyboard, buffer: &Vec<u8>) -> ProtoResponse {
    let mut proto_response = ProtoResponse::from_state(ProtoResponseState::ResultError);
    let mut buffer_ptr = 0;
    if buffer.len() == 0 {
        log::e(TAG, "bad reqeust: empty buffer. This is a bug: must be handled earlier.");
        return ProtoResponse::from_state(ProtoResponseState::ResultError);
    }
    keyboard.lock_sync();
    //log::d(TAG, &format!("Received buffer size of {"));
    while buffer_ptr < buffer.len() {
        let cmd_byte = buffer[buffer_ptr];
        buffer_ptr += 1;
        let cmd_wrapped = ProtoCmd::from_u8(cmd_byte);
        if cmd_wrapped == None {
            log::e(TAG, &format!("bad request: unknown command {} at pos {}", cmd_byte, buffer_ptr - 1));
            return ProtoResponse::from_state(ProtoResponseState::ResultBadRequest);
        }
        let cmd = cmd_wrapped.unwrap();
        log::d(TAG, &format!("cmd={}", cmd_byte));
        if cmd == ProtoCmd::CmdColors {
            buffer_ptr = proto_handle_colors(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CmdSetColor {
            buffer_ptr = proto_handle_set_color(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CmdAddColor {
            buffer_ptr = proto_handle_add_color(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CmdBrightness {
            buffer_ptr = proto_handle_set_brightness(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CmdSpeed {
            buffer_ptr = proto_handle_set_speed(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CmdMode {
            buffer_ptr = proto_handle_set_mode(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CmdSyncState {
            buffer_ptr = proto_handle_set_lock(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CmdPower {
            buffer_ptr = proto_handle_set_power(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CmdToggle {
            buffer_ptr = proto_handle_toggle_power(keyboard, buffer, buffer_ptr);
        } else if cmd == ProtoCmd::CmdReqModesAvail {
            buffer_ptr = proto_handle_request_modes(keyboard, buffer, buffer_ptr,
                                                    &mut proto_response);
        }
        if buffer_ptr == 0 {
            log::e(TAG, "proto_handle_message: parsing message failed.");
            return ProtoResponse::from_state(ProtoResponseState::ResultBadRequest);
        }
    }
    if proto_response.state != ProtoResponseState::ResultData {
        log::d(TAG, "Response state not data, setting to state ok");
        proto_response = ProtoResponse::from_state(ProtoResponseState::ResultOk);
    }
    keyboard.save_state();
    keyboard.unlock_sync();
    keyboard.sync();
    proto_response
}


