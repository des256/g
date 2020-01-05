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

class Headers(QWidget):

    def __init__(self,app,track):

        super().__init__()

        self.app = app
        self.track = track

        # CONSTANTS
        self.WIDTH = 200

        # STATE

        self.setFixedWidth(self.WIDTH)

        self.painter = QPainter()

    def paintEvent(self,event):

        # get visible rectangle
        vx = event.rect().x()
        vy = event.rect().y()
        vw = event.rect().width()
        vh = event.rect().height()

        self.painter.begin(self)

        self.painter.setFont(self.app.font)

        # separator line at the right
        self.painter.setPen(QColor(0x1F,0x1F,0x1F))
        self.painter.drawLine(self.WIDTH - 1,vy,self.WIDTH - 1,vy + vh)

        y = 0
        for channel in self.track.get_channels():

            h = channel.get_height()

            # local space
            ly = y - self.track.get_channel_vo()

            # check if channel is atleast partially visible
            if (ly + h >= vy) and (ly < vy + vh):

                # header rect
                self.painter.fillRect(vx,ly,vw - 1,h,QColor(0x3F,0x3F,0x3F))

                # line under the channel
                self.painter.setPen(QColor(0x1F,0x1F,0x1F))
                self.painter.drawLine(vx,ly + h - 1,vw - 1,ly + h - 1)

                # channel name
                self.painter.setPen(QColor(0xBF,0xBF,0xBF))
                self.painter.drawText(3,ly + 30,channel.get_name())

            # next channel
            y += h

        # void, if any
        ly = y - self.track.get_channel_vo()
        if ly < vy + vh:
            self.painter.fillRect(vx,ly,vw,vh - ly,QColor(0x1F,0x1F,0x1F))

        self.painter.end()
