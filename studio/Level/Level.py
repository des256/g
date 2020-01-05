#!/usr/bin/python3
# -*- coding: utf-8 -*-

# g/Tool
# by Desmond Germans, 2019

import sys
from PyQt5.QtWidgets import *
from PyQt5.QtGui import *
from PyQt5.QtCore import *

import os.path
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(os.path.realpath(__file__)),'..')))

from Main import *

class Level(Main):

    def __init__(self):

        super().__init__('Level Designer','level.png')

    def new_asset(self):

        print('TODO: level.new_asset')

    def open_asset(self):

        print('TODO: level.open_asset')

if __name__ == '__main__':

    application = QApplication(sys.argv)
    level = Level()
    result = application.exec_()
    sys.exit(result)
