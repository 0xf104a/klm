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

static LOGLVL: u8 = 0b01111000;

pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
    PANIC,
}

impl LogLevel {
    fn to_s(&self) -> &str {
        match *self {
            LogLevel::DEBUG => "D",
            LogLevel::INFO  => "I",
            LogLevel::WARN  => "W",
            LogLevel::ERROR => "E",
            LogLevel::PANIC => "P",
        }
    }

    pub fn to_u8(&self) -> u8 {
        match *self {
            LogLevel::DEBUG => 0b10000000,
            LogLevel::INFO  => 0b01000000,
            LogLevel::WARN  => 0b00100000,
            LogLevel::ERROR => 0b00010000,
            LogLevel::PANIC => 0b00001000,
        }
    }
}

fn is_present(level: &LogLevel) -> bool{
    if LOGLVL & level.to_u8() != 0 {
        true
    } else {
        false
    }
}

pub fn log_print(level: LogLevel, tag: &str, msg: &str){
    if !is_present(&level){
        return ;
    }
    println!("{level}/{tag}: {msg}", level=level.to_s(), tag=tag, msg=msg);
}

pub fn d(tag: &str, msg: &str){
    log_print(LogLevel::DEBUG, tag, msg);
}

pub fn i(tag: &str, msg: &str){
    log_print(LogLevel::INFO, tag, msg);
}

pub fn w(tag: &str, msg: &str){
    log_print(LogLevel::WARN, tag, msg);
}

pub fn e(tag: &str, msg: &str){
    log_print(LogLevel::ERROR, tag, msg);
}

pub fn panic(tag: &str, msg: &str){
    log_print(LogLevel::PANIC, tag, msg);
    panic!("Program panicked because of module: {module}", module=tag);
}


