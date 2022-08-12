 # This file is part of KLM project.
 #
 #  Copyright 2022 by Polar <toddot@protonmail.com>
 #
 #  Licensed under GNU General Public License 3.0 or later.
 #  Some rights reserved. See COPYING, AUTHORS.
 #
 # @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>

 # @doc
 # This script sets rainbow colors to keyboard and enables
 # colorshift mode.

from pyklm.connection import KLMConnection
from pyklm.rgb import RGB
from pyklm.mode import KeyboardMode

#FIXME: update this example when pyklm will support
#       sending color vector

# List of rainbow colors
RAINBOW = [RGB(0xFF, 0x00, 0x00), RGB(0xFF, 0xA5, 0x00),
           RGB(0xFF, 0xFF, 0x00), RGB(0x00, 0x80, 0x00), RGB(0x00, 0x00, 0xFF), RGB(0x4B, 0x00, 0x82),
           RGB(0xEE, 0x80, 0xEE)]

# Create connection
connection = KLMConnection()
# Set first color
connection.set_color(RAINBOW[0])
# Add other colors
for color in RAINBOW[1:]: # no need to add 0-th one, we have just added it
    connection.add_color(color)
# Set mode
connection.set_mode(KeyboardMode.MODE_COLORSHIFT)
# Send everything to klmd
connection.commit()
