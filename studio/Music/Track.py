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

from Model import *

class Track(Model):

    class Channel:

        class Clip:

            class Note:

                def __init__(self,x,s,key,velocity):

                    self.x = x # position of the note relative to the clip (b44)
                    self.s = s # length of the note (b44)
                    self.key = key
                    self.velocity = velocity

                def get_pos(self):
                    
                    return self.x

                def set_pos(self,x):

                    self.x = x

                def get_length(self):

                    return self.s

                def set_length(self,s):
                    
                    self.s = s

                def get_key(self):

                    return self.key

                def set_key(self,key):

                    self.key = key

                def get_velocity(self):

                    return self.velocity

                def set_velocity(self,velocity):

                    self.velocity = velocity

            def __init__(self,channel,pos,length):

                super().__init__()

                self.channel = channel
                self.pos = pos
                self.length = length

                self.x0 = 0

                self.xz = 400.0

                self.notes = []
                
                self.notes.append(Track.Channel.Clip.Note(5,3,48,1.0))
                self.notes.append(Track.Channel.Clip.Note(8,3,50,1.0))
                self.notes.append(Track.Channel.Clip.Note(11,3,52,1.0))
                self.notes.append(Track.Channel.Clip.Note(14,3,48,1.0))

            def get_channel(self):

                return self.channel

            def get_pos(self):

                return self.pos

            def set_pos(self):

                self.pos = pos
                
            def get_length(self):

                return self.length

            def set_length(self,length):

                self.length = length

            def get_clip_vo(self):

                return self.x0

            def set_clip_vo(self,x0):

                self.x0 = x0

            def get_clip_vz(self):

                return self.xz

            def set_clip_vz(self,xz):

                self.xz = xz

            def get_notes(self):

                return self.notes

            def x_to_pos(self,x):

                return (x + self.x0) / self.xz

            def dx_to_length(self,dx):

                return dx / self.xz

            def pos_to_x(self,pos):

                return int((pos * self.xz) - self.x0)

            def length_to_dx(self,length):

                return int(length * self.xz)

        def __init__(self,track,name):

            self.track = track
            self.name = name

            # STATE
            self.height = track.CHANNEL_BASE_HEIGHT
            self.y0 = 0 # topmost visible key coordinate in the clip editor

            self.yz = 15.0 # key zoom

            self.clips = []

        def get_name(self):

            return self.name

        def set_name(self,name):

            self.name = name
            
        def get_clips(self):

            return self.clips

        def get_height(self):

            return self.height

        def set_height(self,height):

            self.height = height

        def get_key_vo(self):

            return self.y0

        def set_key_vo(self,y0):

            self.y0 = y0

        def get_key_vz(self):

            return self.yz

        def set_key_vz(self,yz):

            self.yz = yz

        def y_to_key(self,y):

            return 95 - ((y + self.y0) / self.yz)

        def key_to_y(self,key):

            return int(((95 - key) * self.yz) - self.y0)

    def __init__(self,app):

        super().__init__()

        self.app = app

        self.channels = [] # [TrackModel.Channel]

        self.CHANNEL_BASE_HEIGHT = 80

        self.c0 = 0 # topmost visible channel coordinate in the track editor (pixels)
        self.t0 = 0 # leftmost visible track coordinate in the track editor (pixels)
        self.tz = 10.0 # track zoom (pixels per b44)
        self.length = 18 + 50 * 8 # length of the track (b44)

        self.channel = None # the current channel
        self.clip = None # the current clip

        # add a few test channels and clips
        self.channels.append(Track.Channel(self,'Hello'))
        self.channels.append(Track.Channel(self,'World'))
        self.channels.append(Track.Channel(self,'These'))
        self.channels.append(Track.Channel(self,'Are'))
        self.channels.append(Track.Channel(self,'Some'))
        self.channels.append(Track.Channel(self,'Funky'))
        self.channels.append(Track.Channel(self,'Channels'))
        self.channels[0].clips.append(Track.Channel.Clip(self.channels[0],0,16))
        self.channels[1].clips.append(Track.Channel.Clip(self.channels[1],1,4))
        self.channels[2].height = self.CHANNEL_BASE_HEIGHT + 200

        for i in range(0,50):
            self.channels[i % 7].clips.append(Track.Channel.Clip(self.channels[i % 7],18 + i * 8,8))

    def get_channels(self):

        return self.channels

    def get_channel_vo(self):

        return self.c0

    def set_channel_vo(self,c0):

        self.c0 = c0

    def get_track_vo(self):

        return self.t0

    def set_track_vo(self,t0):

        self.t0 = t0

    def get_track_vz(self):

        return self.tz

    def set_track_vz(self,tz):

        self.tz = tz

    def get_length(self):

        return self.length

    def set_length(self,length):

        self.length = length

    def get_current_channel(self):

        return self.channel

    def set_current_channel(self,channel):

        self.channel = channel

    def get_current_clip(self):

        return self.clip

    def set_current_clip(self,clip):

        self.channel = clip.channel
        self.clip = clip

    def x_to_pos(self,x):

        return (x + self.t0) / self.tz

    def pos_to_x(self,pos):

        return int((pos * self.tz) - self.t0)

    def length_to_dx(self,length):

        return int(length * self.tz)

    def y_to_c(self,y):

        return y + self.c0

    def c_to_y(self,c):

        return c - self.c0
