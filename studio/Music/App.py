#!/usr/bin/python3
# -*- coding: utf-8 -*-

# g/Tool
# by Desmond Germans, 2019

import sys
from PyQt5.QtWidgets import *
from PyQt5.QtGui import *
from PyQt5.QtCore import *
import pye

import os.path
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(os.path.realpath(__file__)),'..')))

from MainApp import *
from Track import *
from Headers import *
from TrackEditor import *
from Piano import *
from ClipEditor import *
from Faders import *
from SynthEditor import *

class App(MainApp):

    def __init__(self,audio):

        super().__init__('Music Composer','music.png')

        self.audio = audio

        # general font
        self.font = QFont('sans',10)

        # the editing model: self.model
        self.track = Track(self)

        # playbar
        # TODO: currently a placeholder for a more elaborate play/pause/loop/etc. bar
        self.playbar = QSlider(Qt.Horizontal)

        # edit area:
        #   self.channel_scrollbar = scrollbar to scroll through the channels
        #   self.headers = Headers()
        #   self.track_editor = TrackEditor()
        #   self.track_scrollbar = scrollbar to scroll through the track
        self.edit_area = QStackedWidget()
        track_widget = QWidget()
        track_layout = QGridLayout()
        track_layout.setSpacing(0)
        track_layout.setContentsMargins(0,0,0,0)
        self.channel_scrollbar = QScrollBar(Qt.Vertical)
        self.channel_scrollbar.valueChanged.connect(self.channel_scrollbar_changed)
        track_layout.addWidget(self.channel_scrollbar,0,0)
        self.headers = Headers(self,self.track)
        track_layout.addWidget(self.headers,0,1)
        self.track_editor = TrackEditor(self,self.track)
        track_layout.addWidget(self.track_editor,0,2)
        self.track_scrollbar = QScrollBar(Qt.Horizontal)
        self.track_scrollbar.valueChanged.connect(self.track_scrollbar_changed)
        track_layout.addWidget(self.track_scrollbar,1,2)
        self.track_zoom = QSlider(Qt.Horizontal)
        self.track_zoom.setMinimum(1)
        self.track_zoom.setMaximum(100)
        self.track_zoom.valueChanged.connect(self.track_zoom_changed)
        track_layout.addWidget(self.track_zoom,2,2)
        track_widget.setLayout(track_layout)
        self.edit_area.addWidget(track_widget)

        #   self.key_scrollbar = scrollbar to scroll throught the keys
        #   self.piano = Piano()
        #   self.clip_editor = SegmentEditor()
        #   self.clip_scrollbar = scrollbar to scroll through the clip
        clip_widget = QWidget()
        clip_layout = QGridLayout()
        clip_layout.setSpacing(0)
        clip_layout.setContentsMargins(0,0,0,0)
        self.key_zoom = QSlider(Qt.Vertical)
        self.key_zoom.setMinimum(1)
        self.key_zoom.setMaximum(100)
        self.key_zoom.valueChanged.connect(self.key_zoom_changed)
        clip_layout.addWidget(self.key_zoom,0,0)
        self.key_scrollbar = QScrollBar(Qt.Vertical)
        self.key_scrollbar.valueChanged.connect(self.key_scrollbar_changed)
        clip_layout.addWidget(self.key_scrollbar,0,1)
        self.piano = Piano(self,self.audio,self.track)
        clip_layout.addWidget(self.piano,0,2)
        self.clip_editor = ClipEditor(self,self.track)
        clip_layout.addWidget(self.clip_editor,0,3)
        self.clip_scrollbar = QScrollBar(Qt.Horizontal)
        self.clip_scrollbar.valueChanged.connect(self.clip_scrollbar_changed)
        clip_layout.addWidget(self.clip_scrollbar,1,3)
        self.clip_zoom = QSlider(Qt.Horizontal)
        self.clip_zoom.setMinimum(1)
        self.clip_zoom.setMaximum(100)
        self.clip_zoom.valueChanged.connect(self.clip_zoom_changed)
        clip_layout.addWidget(self.clip_zoom,2,3)
        clip_widget.setLayout(clip_layout)
        self.edit_area.addWidget(clip_widget)

        # faders area: self.faders = Faders()
        faders_area = QScrollArea()
        faders_area.setHorizontalScrollBarPolicy(Qt.ScrollBarAsNeeded)
        faders_area.setVerticalScrollBarPolicy(Qt.ScrollBarAsNeeded)
        faders_area.setWidgetResizable(True)
        self.faders = Faders(self,self.track)
        faders_area.setWidget(self.faders)

        # synth area: self.synth_editor = SynthEditor()
        self.synth_editor = SynthEditor(self,self.track)

        # overall splitter
        self.splitter = QSplitter(Qt.Vertical)
        self.splitter.setChildrenCollapsible(False)
        self.splitter.addWidget(self.edit_area)
        self.splitter.addWidget(faders_area)
        self.splitter.addWidget(self.synth_editor)
        self.splitter.splitterMoved.connect(self.splitter_moved)
        total = self.splitter.height()
        self.splitter.setSizes([total / 3,total / 3,total / 3])

        # main widget
        main_widget = QWidget()
        main_layout = QVBoxLayout()
        main_layout.setSpacing(0)
        main_layout.setContentsMargins(0,0,0,0)
        main_layout.addWidget(self.playbar)
        main_layout.addWidget(self.splitter)
        main_widget.setLayout(main_layout)
        self.setCentralWidget(main_widget)

        # initialize everything
        self.recalc_channel_scrollbar()
        self.recalc_track_scrollbar()

    def recalc_channel_scrollbar(self):

        total = 0
        for channel in self.track.get_channels():
            total += channel.get_height()
        self.channel_scrollbar.setMinimum(0)
        step = self.headers.height()
        self.channel_scrollbar.setPageStep(step)
        if total > step:
            self.channel_scrollbar.setEnabled(True)
            self.channel_scrollbar.setMaximum(total - step)
        else:
            self.channel_scrollbar.setEnabled(False)
            self.channel_scrollbar.setMaximum(0)
        self.headers.update()
        self.track_editor.update()

    def recalc_track_scrollbar(self):

        total = int(self.track.get_length() * self.track.get_track_vz())
        self.track_scrollbar.setMinimum(0)
        step = self.track_editor.width()
        self.track_scrollbar.setPageStep(step)
        if total > step:
            self.track_scrollbar.setEnabled(True)
            self.track_scrollbar.setMaximum(total - step)
        else:
            self.track_scrollbar.setEnabled(False)
            self.track_scrollbar.setMaximum(0)
        self.track_editor.update()

    def recalc_key_scrollbar(self):

        channel = self.track.get_current_channel()
        clip = self.track.get_current_clip()
        if channel and clip:
            total = int(96 * channel.get_key_vz())
            self.key_scrollbar.setMinimum(0)
            step = self.piano.height()
            self.key_scrollbar.setPageStep(step)
            if total > step:
                self.key_scrollbar.setEnabled(True)
                self.key_scrollbar.setMaximum(total - step)
            else:
                self.key_scrollbar.setEnabled(False)
                self.key_scrollbar.setMaximum(0)
            self.piano.update()
            self.clip_editor.update()

    def recalc_clip_scrollbar(self):

        channel = self.track.get_current_channel()
        clip = self.track.get_current_clip()
        if channel and clip:
            total = int(clip.get_length() * clip.get_clip_vz())
            self.clip_scrollbar.setMinimum(0)
            step = self.clip_editor.width()
            self.clip_scrollbar.setPageStep(step)
            if total > step:
                self.clip_scrollbar.setEnabled(True)
                self.clip_scrollbar.setMaximum(total - step)
            else:
                self.clip_scrollbar.setEnabled(False)
                self.clip_scrollbar.setMaximum(0)
            self.clip_editor.update()

    def resizeEvent(self,event):

        if hasattr(self,'track'):
            self.recalc_channel_scrollbar()
            self.recalc_track_scrollbar()
            self.recalc_key_scrollbar()
            self.recalc_clip_scrollbar()

    def wheelEvent(self,event):

        if self.edit_area.currentIndex() == 0:
            value = self.channel_scrollbar.value()
            self.channel_scrollbar.setValue(value - event.angleDelta().y() / 5)
        else:
            value = self.key_scrollbar.value()
            self.key_scrollbar.setValue(value - event.angleDelta().y() / 5)

    def splitter_moved(self,pos,index):

        if index == 1:
            self.recalc_channel_scrollbar()
            self.recalc_key_scrollbar()

    def channel_scrollbar_changed(self):

        self.track.set_channel_vo(self.channel_scrollbar.value())
        self.headers.update()
        self.track_editor.update()

    def track_scrollbar_changed(self):

        self.track.set_track_vo(self.track_scrollbar.value())
        self.track_editor.update()

    def key_scrollbar_changed(self):

        channel = self.track.get_current_channel()
        clip = self.track.get_current_clip()
        if channel and clip:
            channel.set_key_vo(self.key_scrollbar.value())
            self.piano.update()
            self.clip_editor.update()

    def clip_scrollbar_changed(self):

        channel = self.track.get_current_channel()
        clip = self.track.get_current_clip()
        if channel and clip:
            clip.set_clip_vo(self.clip_scrollbar.value())
            self.clip_editor.update()

    def track_zoom_changed(self):

        self.track.set_track_vz(self.track_zoom.value())
        self.recalc_track_scrollbar()

    def key_zoom_changed(self):

        channel = self.track.get_current_channel()
        clip = self.track.get_current_clip()
        if channel and clip:
            channel.set_key_vz(self.key_zoom.value() * 5)
            self.recalc_key_scrollbar()

    def clip_zoom_changed(self):

        channel = self.track.get_current_channel()
        clip = self.track.get_current_clip()
        if channel and clip:
            clip.set_clip_vz(self.clip_zoom.value() * 20)
            self.recalc_clip_scrollbar()

    def switch_to_track(self):

        self.edit_area.setCurrentIndex(0)

    def switch_to_clip(self,clip):

        self.edit_area.setCurrentIndex(1)

    def keyPressEvent(self,event):

        key = event.key()
        if key == Qt.Key_Escape:
            if self.edit_area.currentIndex() == 1:
                self.switch_to_track()
                return
        super(App,self).keyPressEvent(event)

    def new_asset(self):

        print('TODO: new_asset()')

    def open_asset(self):

        print('TODO: open_asset()')

if __name__ == '__main__':

    audio = pye.create_audio()

    synth = pye.create_dogma()
    audio.edit_set_synth(synth)

    application = QApplication(sys.argv)
    app = App(audio)
    result = application.exec_()
    sys.exit(result)
