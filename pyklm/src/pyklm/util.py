 # This file is part of pyklm project.
 #
 #  Copyright 2022 by Polar <toddot@protonmail.com>
 #
 #  Licensed under GNU General Public License 3.0 or later.
 #  Some rights reserved. See COPYING, AUTHORS.
 #
 # @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>

def byteargs(func):
    """
     Wrapper that ensures that all int arguments of
     a function are in byte range(0-255)
    """
    def wrapper(*args):
        for arg in args:
            if isinstance(arg, int):
                if arg < 0 or arg > 255:
                    raise ValueError("Argument value %d is out of byte range" % arg)
        return func(*args)
    return wrapper



