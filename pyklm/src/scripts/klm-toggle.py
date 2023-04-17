#!/usr/bin/env python
# This file is part of pyklm project.
#
#  Copyright 2023 by Polar <toddot@protonmail.com>
#
#  Licensed under GNU General Public License 3.0 or later.
#  Some rights reserved. See COPYING, AUTHORS.
#
# @license GPL-3.0+ <http://spdx.org/licenses/GPL-3.0+>
# @doc Simple utility to toggle keyboard lightning
import sys

from pyklm.connection import KLMConnection, KLMResultStatus

def main() -> int:
    connection = KLMConnection()
    connection.toggle()
    result = connection.commit()
    if result.status != KLMResultStatus.RESULT_OK:
        print(f"Failed to toggle keyboard lightning: klmd returned {result.status} instead of RESULT_OK")
        return -1
    return 0

if __name__ == '__main__':
    sys.exit(main())