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

class Sound(Main):

    def __init__(self):

        super().__init__('Sound Effect Studio','sound.png')

    def new_asset(self):

        print('TODO: sound.new_asset()')

    def open_asset(self):

        print('TODO: sound.open_asset()')
        
if __name__ == '__main__':

    application = QApplication(sys.argv)
    sound = Sound()
    result = application.exec_()
    sys.exit(result)
