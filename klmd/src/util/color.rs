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

pub struct RGB{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB{
    pub fn new(_r: u8, _g: u8, _b: u8) -> RGB{
        RGB{
            r: _r,
            g: _g,
            b: _b,
        }
    }
    pub fn to_s(&self) -> String{
        format!("<RGB: {}, {}, {}>", self.r, self.g, self.b)
    }
}
