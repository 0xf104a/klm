 # This file is part of KLM project.
 #
 #  Copyright 2022 by Polar <toddot@protonmail.com>
 #
 #  Licensed under GNU General Public License 3.0 or later.
 #  Some rights reserved. See COPYING, AUTHORS.
 #
 # @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>

# @doc
# Example script
# Sets keyboard color to blue, mode to steady and brightness to 10
from pyklm.connection import KLMConnection
from pyklm.mode import KeyboardMode
from pyklm.rgb import RGB

# Create connection
connection = KLMConnection()
# Stage command
connection.set_color(RGB(0, 0, 255))
# Set brightness
connection.set_brightness(0xA)
# Set mode
connection.set_mode(KeyboardMode.MODE_STEADY)
# Send command to daemon
connection.commit()
