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

class ClipEditor(QWidget):

    class Mode:

        NOTHING = 0
        MOVING = 1
        GROWING_START = 2
        GROWING_END = 3

    class Hit:

        NOTHING = 0
        NOTE = 1
        START = 2
        END = 3

    def __init__(self,app,track):

        super().__init__()
        self.app = app
        self.track = track

        self.mode = ClipEditor.Mode.NOTHING
        self.hit = ClipEditor.Hit.NOTHING
        self.note = None
        self.length = 0
        self.dx = 0
        self.dy = 0

        self.setMouseTracking(True)

        self.painter = QPainter()

    def paintEvent(self,event):

        # get visible rectangle
        vx = event.rect().x()
        vy = event.rect().y()
        vw = event.rect().width()
        vh = event.rect().height()

        # start painting
        self.painter.begin(self)

        #self.painter.fillRect(0,0,self.width(),self.height(),QColor(0x3F,0x3F,0x3F))

        channel = self.track.get_current_channel()
        clip = self.track.get_current_clip()
        if channel and clip:

            h = int(channel.get_key_vz())

            # background pattern
            for key in range(0,96):

                # local space
                y = channel.key_to_y(key)

                # check if key is atleast partially visible
                if (y + h >= vy) and (y < vy + vh):

                    k = key % 12
                    if (k == 0) or (k == 2) or (k == 4) or (k == 5) or (k == 7) or (k == 9) or (k == 11):
                        key_color = QColor(0x5F,0x5F,0x5F)
                    else:
                        key_color = QColor(0x3F,0x3F,0x3F)

                    # key background rect
                    xend = clip.pos_to_x(clip.get_length())
                    if xend > vw:
                        xend = vw
                    self.painter.fillRect(vx,y,xend - vx,h,key_color)

                    # void, if any
                    if xend < vw:
                        self.painter.fillRect(xend,y,vw - xend,h,QColor(0x1F,0x1F,0x1F))

            # notes
            rx = clip.x_to_pos(vx)
            rl = clip.dx_to_length(vw)
            self.painter.setPen(QColor(0x00,0x00,0x00))
            for note in clip.get_notes():
                color = QColor(0xBF,0xBF,0xBF)
                if note == self.note:
                    if self.mode != ClipEditor.Mode.NOTHING:
                        color = QColor(0x7F,0x7F,0xFF)
                    else:
                        color = QColor(0xBF,0xBF,0xFF)
                p = note.get_pos()
                l = note.get_length()
                if (p >= rx) and (p < rx + rl):
                    key = note.get_key()
                    y = channel.key_to_y(key)
                    x = clip.pos_to_x(p)
                    s = clip.length_to_dx(l)
                    if (y + h >= vy) and (y < vy + vh):
                        self.painter.drawRect(x,y,s,h)
                        self.painter.fillRect(x + 1,y + 1,s - 2,h - 2,color)

            # void, if any
            ly = channel.key_to_y(0) + h
            if ly < vy + vh:
                self.painter.fillRect(vx,ly,vw,vh - ly,QColor(0x1F,0x1F,0x1F))

        self.painter.end()

    def get_note_at(self,x,y):

        channel = self.track.get_current_channel()
        clip = self.track.get_current_clip()
        if channel and clip:
            pos = clip.x_to_pos(x)
            key = int(channel.y_to_key(y) + 1) # +1 is because the transformation transforms the bottom of the cell, not the top
            for note in clip.get_notes():
                p = note.get_pos()
                l = note.get_length()
                k = note.get_key()
                if (pos >= p) and (pos < p + l) and (key == k):
                    return note
        return None

    def mousePressEvent(self,event):

        channel = self.track.get_current_channel()
        clip = self.track.get_current_clip()

        if event.button() == Qt.LeftButton:

            if self.hit != ClipEditor.Hit.NOTHING:

                cx = clip.pos_to_x(self.note.get_pos())
                cy = channel.key_to_y(self.note.get_key())
                self.dx = event.x() - cx
                self.dy = event.y() - cy
                self.length = self.note.get_length()

                if self.hit == ClipEditor.Hit.NOTE:

                    self.mode = ClipEditor.Mode.MOVING

                elif self.hit == ClipEditor.Hit.START:

                    self.mode = ClipEditor.Mode.GROWING_START

                elif self.hit == ClipEditor.Hit.END:

                    self.mode = ClipEditor.Mode.GROWING_END

                self.update()

    def mouseDoubleClickEvent(self,event):        

        if event.button() == Qt.LeftButton:

            if not self.note:
                # TODO: create new note here
                pass

    def mouseReleaseEvent(self,event):

        if event.button() == Qt.LeftButton:

            self.mode = ClipEditor.Mode.NOTHING

    def mouseMoveEvent(self,event):
        
        channel = self.track.get_current_channel()
        clip = self.track.get_current_clip()

        x = event.x()
        y = event.y()

        if self.mode == ClipEditor.Mode.MOVING:

            pos = int(clip.x_to_pos(x - self.dx))
            key = int(channel.y_to_key(y - self.dy) + 1)

            self.note.set_pos(pos)
            self.note.set_key(key)

        elif self.mode == ClipEditor.Mode.GROWING_START:

            pos = int(clip.x_to_pos(x - self.dx))
            end = self.note.get_pos() + self.note.get_length()
            self.note.set_pos(pos)
            self.note.set_length(end - pos)

        elif self.mode == ClipEditor.Mode.GROWING_END:

            pos = self.note.get_pos()
            end = int(clip.x_to_pos(x - self.dx)) + self.length
            self.note.set_length(end - pos)

        else:

            self.note = self.get_note_at(event.x(),event.y())

            if self.note:

                x1 = clip.pos_to_x(self.note.get_pos())
                x2 = x1 + clip.length_to_dx(self.note.get_length())
                if (x >= x1) and (x < x1 + 5):
                    self.hit = ClipEditor.Hit.START
                    self.setCursor(QCursor(Qt.SplitHCursor))
                elif (x >= x2 - 5) and (x < x2):
                    self.hit = ClipEditor.Hit.END
                    self.setCursor(QCursor(Qt.SplitHCursor))
                else:
                    self.hit = ClipEditor.Hit.NOTE
                    self.setCursor(QCursor(Qt.PointingHandCursor))

            else:

                self.hit = ClipEditor.Hit.NOTHING
                self.setCursor(QCursor(Qt.ArrowCursor))

        self.update()