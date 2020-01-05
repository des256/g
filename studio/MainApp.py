#!/usr/bin/python3
# -*- coding: utf-8 -*-

# e studio generic: main application base class
# by Desmond Germans, 2019

import sys
import os
from PyQt5.QtWidgets import *
from PyQt5.QtGui import *
from PyQt5.QtCore import *

class MainApp(QMainWindow):

    def __init__(self,title,icon):

        super().__init__()

        self.title = title
        self.icon = icon

        self.asset = None
        self.name = None
        self.state = 0
        self.new_state = 0
        self.saved_state = 0

        self.undos = []
        self.redos = []

        self.recents = []

        self.init_actions()
        self.init_menubar()

        #width = QDesktopWidget().availableGeometry().width()
        #height = QDesktopWidget().availableGeometry().height()
        #print('overall geometry: {}x{}'.format(width,height))
        #self.setGeometry(0,0,width / 2,height / 2)
        self.setGeometry(0,0,1920,1080)
        qr = self.frameGeometry()
        cp = QDesktopWidget().availableGeometry().center()
        qr.moveCenter(cp)
        self.move(qr.topLeft())

        self.setWindowTitle(title + ' - Untitled')
        self.setWindowIcon(self.make_icon(icon))

        self.show()
        self.update_actions()

    def make_icon(self,name):

        here = os.path.realpath(os.path.join(os.path.dirname(os.path.abspath(sys.argv[0])),'icons'))
        parent = os.path.join(os.path.realpath(os.path.join(os.path.dirname(os.path.abspath(sys.argv[0])),'..')),'icons')
        here_name = os.path.join(here,name)
        parent_name = os.path.join(parent,name)
        if os.path.exists(here_name):
            return QIcon(here_name)
        if os.path.exists(parent_name):
            return QIcon(parent_name)
        print('unable to find {} in {} or {}'.format(name,here,parent))
        return QIcon()

    def new_action(self,name,icon,shortcut,tip,triggered):

        action = QAction(self.make_icon(icon),name,self)
        if shortcut:
            action.setShortcut(shortcut)
        action.setStatusTip(tip)
        action.triggered.connect(triggered)
        return action

    def init_actions(self):

        self.actions_new = self.new_action('New','new.png','Ctrl+N','New',self.new_asset)
        self.actions_open = self.new_action('Open','open.png','Ctrl+O','Open...',self.open_asset)
        self.actions_revert = self.new_action('Revert','revert.png','Ctrl+R','Revert',self.revert_asset)
        self.actions_save = self.new_action('Save','save.png','Ctrl+S','Save',self.save_asset)
        self.actions_save_as = self.new_action('Save As','save_as.png','Shift+S','Save As...',self.save_asset_as)
        self.actions_exit = self.new_action('Exit','exit.png','Ctrl+Q','Exit',self.close)
        self.actions_undo = self.new_action('Undo','undo.png','Ctrl+Z','Undo Last Action',self.undo)
        self.actions_redo = self.new_action('Redo','redo.png','Ctrl+Y','Redo Last Undone Action',self.redo)

    def init_menubar(self):
        
        menubar = self.menuBar()

        self.asset_menu = menubar.addMenu('&File')
        self.asset_menu.addAction(self.actions_new)
        self.asset_menu.addAction(self.actions_open)
        self.recent_menu = self.asset_menu.addMenu('Open Recent...')
        self.asset_menu.addSeparator()
        self.asset_menu.addAction(self.actions_save)
        self.asset_menu.addAction(self.actions_save_as)
        self.asset_menu.addSeparator()
        self.asset_menu.addAction(self.actions_revert)
        self.asset_menu.addSeparator()
        self.asset_menu.addAction(self.actions_exit)

        self.edit_menu = menubar.addMenu('&Edit')
        self.edit_menu.addAction(self.actions_undo)
        self.edit_menu.addAction(self.actions_redo)

        self.help_menu = menubar.addMenu('&Help')

    def update_actions(self):

        self.actions_revert.setEnabled(self.name is not None)
        self.actions_save.setEnabled(self.state != self.saved_state)
        self.actions_undo.setEnabled(len(self.undos) > 0)
        self.actions_redo.setEnabled(len(self.redos) > 0)

    def new_asset(self):

        print('new_asset should be implemented on the daughter class')

    def open_asset(self):

        print('open_asset should be implemented on the daughter class')
        
    def revert_asset(self):

        if not self.name:
            return

        # TODO: verify user really wants to revert

        self.state = 0
        self.new_state = 0
        self.saved_state = 0
        self.undos = []
        self.redos = []

        # TODO: load asset self.name

        self.asset.update()
        self.update()

    def save_asset(self):

        if not self.name:

            self.save_asset_as()

        else:

            self.asset.save(self.name)
            self.saved_state = self.state

        self.update()

    def save_asset_as(self):

        (name,sel) = QFileDialog.getSaveFileName(parent=self,caption='Save ' + self.title + ' As')
        if not name:
            return

        self.save_asset()

    def apply(self,command):

        self.new_state += 1

        command()

        command.set_states(self.state,self.new_state)

        self.undos.append(command)
        self.redos.clear()

        self.state = self.new_state

        self.asset.update()
        self.update()

    def undo(self):

        if len(self.undos) > 0:

            command = self.undos.pop()
            command.undo()

            self.state = command.before_state

            self.redos.append(command)

            self.asset.update()
            self.update()

    def redo(self):

        if len(self.redos) > 0:

            command = self.redos.pop()
            command()

            self.state = command.after_state

            self.undos.append(command)

            self.asset.update()
            self.update()
