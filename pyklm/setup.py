#!/usr/bin/env python
import sys

import setuptools
import os

sys.path.append(os.path.join(os.path.dirname(os.path.realpath(__file__)), "src"))

from pyklm import __version__

if __name__ == "__main__":
    setuptools.setup(version=__version__)
