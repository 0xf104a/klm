 # This file is part of pyklm project.
 #
 #  Copyright 2022 by Polar <toddot@protonmail.com>
 #
 #  Licensed under GNU General Public License 3.0 or later.
 #  Some rights reserved. See COPYING, AUTHORS.
 #
 # @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>

from pyklm.util import byteargs

class RGB:
    """
     Stores RGB color compatiable with klmd
    """
    @byteargs
    def __init__(self, r: int, g: int, b: int):
        """
         Initializes RGB color.

         :param r: Red color itensity(0-255)
         :param g: Green color itensity(0-255)
         :param b: Blue color itensit(0-255)
        """
        self.r = r
        self.g = g
        self.b = b

    def to_bytearray(self) -> bytearray:
        """
         Converts RGB color to byte sequence of color itensties

         :return bytearray: byte sequence of color
        """
        return bytearray([self.r, self.g, self.b])

