 # This file is part of pyklm project.
 #
 #  Copyright 2022 by Polar <toddot@protonmail.com>
 #
 #  Licensed under GNU General Public License 3.0 or later.
 #  Some rights reserved. See COPYING, AUTHORS.
 #
 # @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>

import socket
import os
from enum import Enum

from pyklm.rgb import RGB
from pyklm.util import byteargs
from pyklm.mode import KeyboardMode

class KLMError(Exception):
    pass

class KLMResult(Enum):
    RESULT_OK = 0x0
    RESULT_ERROR = 0x1
    RESULT_BAD_REQUEST = 0x2

    @byteargs
    @staticmethod
    def from_byte(byte: int):
        if byte == 0x0:
            return KLMResult.RESULT_OK
        elif byte == 0x1:
            return KLMResult.RESULT_ERROR
        elif byte == 0x2:
            return KLMResult.RESULT_BAD_REQUEST
        else:
            raise ValueError(f"Bad status code: {byte}")

class KLMConnection:
    """
     Stores data required to interact with klmd
    """

    def __init__(self):
        self.staged = bytearray()
        self.size = 0

    def reset(self):
        """
         Resets connection for re-use
        """
        self.stage = bytearray()
        self.size = 0

    def set_color(self, color: RGB):
        """
         Stages set color command.

         :param color: RGB: color to set
        """
        self.staged += bytearray([0x01])
        self.staged += color.to_bytearray()
        self.size += 4

    @byteargs
    def set_brightness(self, brightness: int):
        """
         Stages set brightness command.

         :param brightness: int: brightness level(0-255)
        """
        self.staged += bytearray([0x03])
        self.staged += bytearray([brightness])
        self.size += 2

    def set_mode(self, mode: KeyboardMode):
        """
         Stages command to setting mode

         :param mode: KeyboardMode: mode to use
        """
        self.staged += bytearray([0x05])
        self.staged += bytearray([mode.value])
        self.size += 2

    def add_color(self, color: RGB):
        """
         Stages add color command.

         :param color: RGB: color to add
        """
        self.staged += bytearray([0x02])
        self.staged += color.to_bytearray()
        self.size += 4

    def commit(self) -> KLMResult:
        """
         Commits staged changes to daemon.

         :return: KLMResult: result of communication.
        """
        if not os.path.exists("/var/run/klmd.sock"):
            raise KLMError("No sock found. Is daemon running?")
        if self.size == 0:
            raise KLMError("No commands staged. If you have stage commands before this may be a bug.")
        if self.size > 255:
            raise KLMError(f"Size of requst {self.size} is too big. Try reducing amount of commands.")
        sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        sock.connect("/var/run/klmd.sock")
        sock.send(bytearray([self.size]))
        sock.send(self.staged)
        result = KLMResult.from_byte(sock.recv(1)[0])
        sock.close()
        return result

