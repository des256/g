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

class Piano(QWidget):

    def __init__(self,app,audio,track):

        super().__init__()

        self.app = app
        self.audio = audio
        self.track = track

        # CONSTANTS
        self.BACK_WIDTH = 40
        self.FRONT_WIDTH = 20
        self.KEY_HEIGHT_BACK = 10
        self.KEY_HEIGHT_FRONT = (self.KEY_HEIGHT_BACK * 12.0 / 7.0)

        # STATE
        self.key = 0
        self.down = False

        self.setFixedWidth(self.BACK_WIDTH + self.FRONT_WIDTH)

        self.painter = QPainter()

    def paintEvent(self,event):

        # get visible rectangle
        vx = event.rect().x()
        vy = event.rect().y()
        vw = event.rect().width()
        vh = event.rect().height()

        self.painter.begin(self)

        # separator line at the right
        self.painter.setPen(QColor(0x1F,0x1F,0x1F))
        self.painter.drawLine(self.BACK_WIDTH + self.FRONT_WIDTH - 1,vy,self.BACK_WIDTH + self.FRONT_WIDTH - 1,vy + vh)

        channel = self.track.get_current_channel()
        clip = self.track.get_current_clip()
        if channel and clip:

            # heights
            hb = channel.get_key_vz()
            hf = (hb * 12.0) / 7.0            

            yb = 0.0
            yf = 0.0

            for key in range(0,96):

                # separation line color
                self.painter.setPen(QColor(0x1F,0x1F,0x1F))

                # key modulo octave, inverted
                k = 11 - (key % 12)

                # local space
                lyb = yb - channel.get_key_vo()
                lyf = yf - channel.get_key_vo()

                # back side is int
                tb = int(lyb)
                bb = int(lyb + hb)

                # front side is different
                if (k == 0) or (k == 2) or (k == 4) or (k == 5) or (k == 7) or (k == 9) or (k == 11):

                    tf = int(lyf)
                    bf = int(lyf + hf)

                    if (k == 0) or (k == 5):
                        bf = bb
                    elif (k == 4) or (k == 11):
                        tf = tb

                    # white back
                    self.painter.fillRect(0,tb,self.BACK_WIDTH,bb - tb,QColor(0xBF,0xBF,0xBF))
                    self.painter.drawLine(0,bb - 1,self.BACK_WIDTH,bb - 1)

                    # white front
                    self.painter.fillRect(self.BACK_WIDTH,tf,self.FRONT_WIDTH - 1,bf - tf,QColor(0xBF,0xBF,0xBF))
                    self.painter.drawLine(self.BACK_WIDTH,bf - 1,self.BACK_WIDTH + self.FRONT_WIDTH - 1,bf - 1)

                else:

                    # black back
                    self.painter.fillRect(0,tb,self.BACK_WIDTH,bb - tb,QColor(0x3F,0x3F,0x3F))
                    self.painter.drawLine(0,bb - 1,self.BACK_WIDTH,bb - 1)
                    
                # next key
                yb += hb
                if (k == 0) or (k == 2) or (k == 4) or (k == 5) or (k == 7) or (k == 9) or (k == 11):
                    yf += hf

            # void, if any
            lyb = yb - channel.get_key_vo()
            if lyb < vy + vh:
                self.painter.fillRect(vx,lyb,vw,vh - lyb,QColor(0x1F,0x1F,0x1F))

        self.painter.end()

    def get_key_at(self,x,y):

        channel = self.track.get_current_channel()
        clip = self.track.get_current_clip()
        if channel and clip:
            key = int(channel.y_to_key(y) + 1)
            if (key >= 0) and (key < 96):
                return key
        return None

    def mousePressEvent(self,event):

        channel = self.track.get_current_channel()
        clip = self.track.get_current_clip()
        if (event.button() == Qt.LeftButton) and not self.down:
            if self.key:
                self.audio.edit_release(self.key)
            key = self.get_key_at(event.x(),event.y())
            if key:
                self.key = key
                self.down = True
                self.audio.edit_note(self.key,0.8) # TODO: velocity
                self.update()

    def mouseReleaseEvent(self,event):

        if (event.button() == Qt.LeftButton) and self.down:
            if self.key:
                self.audio.edit_release(self.key)
                self.down = False
                self.key = None

    def mouseMoveEvent(self,event):

        pass