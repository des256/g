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

class Faders(QWidget):

    def __init__(self,app,track):

        super().__init__()
        
        self.app = app
        self.track = track

        self.setFixedHeight(400)
        
        self.painter = QPainter()

    def paintEvent(self,event):

        self.painter.begin(self)
        self.painter.fillRect(0,0,self.width(),self.height(),QColor(0x7F,0x7F,0xBF))
        # TODO: app.zoom and app.offset
        self.painter.end()
