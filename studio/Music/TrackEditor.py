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

class TrackEditor(QWidget):

    def __init__(self,app,track):

        super().__init__()

        self.app = app
        self.track = track

        # STATE
        self.pressed = False

        self.painter = QPainter()

    def paintEvent(self,event):

        # get visible rectangle
        vx = event.rect().x()
        vy = event.rect().y()
        vw = event.rect().width()
        vh = event.rect().height()

        # start painting
        self.painter.begin(self)

        self.painter.setFont(self.app.font)

        y = 0

        cvo = self.track.get_channel_vo()
        for channel in self.track.get_channels():

            h = channel.get_height()

            # local space
            ly = self.track.c_to_y(y)
            #y - cvo

            # check if channel is atleast partially visible
            if (ly + h >= vy) and (ly < vy + vh):
            
                # channel rect
                tl = self.track.get_length()
                x1 = self.track.pos_to_x(tl)
                if x1 > vw:
                    x1 = vw
                self.painter.fillRect(vx,ly,x1 - vx,h,QColor(0x3F,0x3F,0x3F))

                # void, if any
                if x1 < vw:
                    self.painter.fillRect(x1,ly,vw - x1,h,QColor(0x1F,0x1F,0x1F))

                # line under the channel
                self.painter.setPen(QColor(0x1F,0x1F,0x1F))
                self.painter.drawLine(vx,ly + h - 1,vx + x1 - 1,ly + h - 1)

                # draw the clips
                for clip in channel.get_clips():

                    # local space
                    p = clip.get_pos()
                    l = clip.get_length()
                    lx = self.track.pos_to_x(p)
                    w = self.track.length_to_dx(l)

                    # check if clip is at least partially visible
                    if (lx + w >= vx) and (lx < vx + vw):

                        # clip rect
                        self.painter.setPen(QColor(0x1F,0x1F,0x1F))
                        self.painter.drawRect(lx,ly,w,h)
                        self.painter.fillRect(lx + 1,ly + 1,w - 2,h - 2,QColor(0xBF,0xBF,0xBF))

            # next channel
            y += h

        # void, if any
        ly = y - cvo
        if ly < vy + vh:
            self.painter.fillRect(vx,ly,vw,vh - ly,QColor(0x1F,0x1F,0x1F))

        self.painter.end()

    def get_clip_at(self,x,y):

        pos = self.track.x_to_pos(x)
        c = self.track.y_to_c(y)
        cy = 0
        for channel in self.track.get_channels():
            h = channel.get_height()
            if (c >= cy) and (c < cy + h):
                for clip in channel.get_clips():
                    p = clip.get_pos()
                    l = clip.get_length()
                    if (pos >= p) and (pos < p + l):
                        return clip
            cy += h
        return None

    def mousePressEvent(self,event):

        if event.button() == Qt.LeftButton:

            if not self.pressed:

                clip = self.get_clip_at(event.x(),event.y())
                if clip:
                    self.track.set_current_clip(clip)
                    self.pressed = True

    def mouseDoubleClickEvent(self,event):        

        if event.button() == Qt.LeftButton:

            clip = self.get_clip_at(event.x(),event.y())
            if clip:
                self.track.set_current_clip(clip)
                self.app.switch_to_clip(clip)

    def mouseReleaseEvent(self,event):

        if event.button() == Qt.LeftButton:

            if self.pressed:

                self.pressed = False

    def mouseMoveEvent(self,event):

        pass