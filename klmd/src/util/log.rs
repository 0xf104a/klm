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
            LogLevel::INFO => "I",
            LogLevel::WARN => "W",
            LogLevel::ERROR => "E",
            LogLevel::PANIC => "P",
        }
    }
}

pub fn log_print(level: LogLevel, tag: &str, msg: &str){
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
