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

class Timeline(Main):

    def __init__(self):

        super().__init__('Timeline Editor','timeline.png')

	def new_asset(self):

		print('TODO: timeline.new_asset()')

	def open_asset(self):

		print('TODO: timeline.open_asset()')

if __name__ == '__main__':

	application = QApplication(sys.argv)
	timeline = Timeline()
	result = application.exec_()
	sys.exit(result)
