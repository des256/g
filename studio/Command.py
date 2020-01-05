#!/usr/bin/python3
# -*- coding: utf-8 -*-

# e studio generic: command base class
# by Desmond Germans, 2019

import sys

class Command:

	def __init__(self):
		self.before_state = 0
		self.after_state = 0

	def __call__(self):
		()

	def Undo(self):
		()

	def SetStates(self,before_state,after_state):
		self.before_state = before_state
		self.after_state = after_state