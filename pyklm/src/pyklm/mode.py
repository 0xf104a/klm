 # This file is part of pyklm project.
 #
 #  Copyright 2022 by Polar <toddot@protonmail.com>
 #
 #  Licensed under GNU General Public License 3.0 or later.
 #  Some rights reserved. See COPYING, AUTHORS.
 #
 # @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>

from enum import Enum

class KeyboardMode(Enum):
    """
     Describes possible keyboard modes.

     MODE_OFF = 0x0 switches off keyboard lightning
     ModeSteady = 0x1 switches on keyboard lightning in steady mode
     ModeBreathing = 0x2 turns on keyboard lightning in breathing mode
     ModeColorshift = 0x3 turns on keyboard lightning in colorshift mode
    """
    MODE_OFF = 0x00
    MODE_STEADY = 0x01
    MODE_BREATHING = 0x02
    MODE_COLORSHIFT = 0x03
