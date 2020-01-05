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

def buildUnit(layout,label,row,col):

    text = QLabel(label)
    text.setAlignment(Qt.AlignCenter | Qt.AlignVCenter)
    layout.addWidget(text,row,col)
    return text

def buildDial(layout,row,col,label,min,max,changed):

    dial = QDial()
    dial.setMinimum(min)
    dial.setMaximum(max)
    dial.valueChanged.connect(changed)
    layout.addWidget(dial,row,col)
    text = QLabel(label)
    text.setAlignment(Qt.AlignCenter | Qt.AlignVCenter)
    layout.addWidget(text,row + 1,col)
    return dial

class SynthEditor(QWidget):

    def __init__(self,app,track):

        super().__init__()
        self.app = app
        self.track = track

        layout = QGridLayout()

        self.save_button = QPushButton('Save')
        layout.addWidget(self.save_button,0,0)
        self.load_button = QPushButton('Load')
        layout.addWidget(self.load_button,0,1)
        self.patch_name = QComboBox()
        layout.addWidget(self.patch_name,0,2,1,4)

        buildUnit(layout,'PWM',1,0)
        buildUnit(layout,'DCO',1,5)
        buildUnit(layout,'HPF',1,11)
        buildUnit(layout,'VCF',1,13)
        buildUnit(layout,'VCA',1,20)

        self.pwm_pulse_dial = buildDial(layout,2,0,'WIDTH',0,100,self.pwmWidthChanged)
        self.pwm_lfo_level_dial = buildDial(layout,2,1,'LFO',0,100,self.pwmLfoLevelChanged)
        self.pwm_env_level_dial = buildDial(layout,2,2,'ENV',-100,100,self.pwmEnvLevelChanged)
        self.pwm_lfo_delay_dial = buildDial(layout,4,0,'DELAY',0,100,self.pwmLfoDelayChanged)
        self.pwm_lfo_rate_dial = buildDial(layout,4,1,'RATE',0,100,self.pwmLfoRateChanged)
        self.pwm_env_attack_dial = buildDial(layout,6,0,'ATT',0,100,self.pwmEnvAttackChanged)
        self.pwm_env_decay_dial = buildDial(layout,6,1,'DEC',0,100,self.pwmEnvDecayChanged)
        self.pwm_env_sustain_dial = buildDial(layout,6,2,'SUS',0,100,self.pwmEnvSustainChanged)
        self.pwm_env_release_dial = buildDial(layout,6,3,'REL',0,100,self.pwmEnvReleaseChanged)

        self.dco_pwm_level_dial = buildDial(layout,2,5,'PWM',0,100,self.dcoPwmLevelChanged)
        self.dco_saw_level_dial = buildDial(layout,2,6,'SAW',0,100,self.dcoSawLevelChanged)
        self.dco_sub_level_dial = buildDial(layout,2,7,'SUB',0,100,self.dcoSubLevelChanged)
        self.dco_noise_level_dial = buildDial(layout,2,8,'NOISE',0,100,self.dcoNoiseLevelChanged)
        self.dco_lfo_level_dial = buildDial(layout,2,9,'LFO',0,100,self.dcoLfoLevelChanged)
        self.dco_lfo_delay_dial = buildDial(layout,4,5,'DELAY',0,100,self.dcoLfoDelayChanged)
        self.dco_lfo_rate_dial = buildDial(layout,4,6,'RATE',0,100,self.dcoLfoRateChanged)

        self.hpf_cutoff_dial = buildDial(layout,2,11,'CUT',0,100,self.hpfCutoffChanged)

        self.vcf_cutoff_dial = buildDial(layout,2,13,'CUT',0,100,self.vcfCutoffChanged)
        self.vcf_resonance_dial = buildDial(layout,2,14,'RESO',0,100,self.vcfResonanceChanged)
        self.vcf_key_sense_dial = buildDial(layout,2,15,'KEY',-100,100,self.vcfKeySenseChanged)
        self.vcf_velocity_dial = buildDial(layout,2,16,'VELO',0,100,self.vcfVelocityChanged)
        self.vcf_lfo_level_dial = buildDial(layout,2,17,'LFO',0,100,self.vcfLfoLevelChanged)
        self.vcf_env_level_dial = buildDial(layout,2,18,'ENV',-100,100,self.vcfEnvLevelChanged)
        self.vcf_lfo_delay_dial = buildDial(layout,4,13,'DELAY',0,100,self.vcfLfoDelayChanged)
        self.vcf_lfo_rate_dial = buildDial(layout,4,14,'RATE',0,100,self.vcfLfoRateChanged)
        self.vcf_env_attack_dial = buildDial(layout,6,13,'ATT',0,100,self.vcfEnvAttackChanged)
        self.vcf_env_decay_dial = buildDial(layout,6,14,'DEC',0,100,self.vcfEnvDecayChanged)
        self.vcf_env_sustain_dial = buildDial(layout,6,15,'SUS',0,100,self.vcfEnvSustainChanged)
        self.vcf_env_release_dial = buildDial(layout,6,16,'REL',0,100,self.vcfEnvReleaseChanged)

        self.vca_key_sense_dial = buildDial(layout,2,20,'KEY',-100,100,self.vcaKeySenseChanged)
        self.vca_velocity_dial = buildDial(layout,2,21,'VELO',0,100,self.vcaVelocityChanged)
        self.vca_lfo_level_dial = buildDial(layout,2,22,'LFO',0,100,self.vcaLfoLevelChanged)
        self.vca_lfo_delay_dial = buildDial(layout,4,20,'DELAY',0,100,self.vcaLfoDelayChanged)
        self.vca_lfo_rate_dial = buildDial(layout,4,21,'RATE',0,100,self.vcaLfoRateChanged)
        self.vca_env_attack_dial = buildDial(layout,6,20,'ATT',0,100,self.vcaEnvAttackChanged)
        self.vca_env_decay_dial = buildDial(layout,6,21,'DEC',0,100,self.vcaEnvDecayChanged)
        self.vca_env_sustain_dial = buildDial(layout,6,22,'SUS',0,100,self.vcaEnvSustainChanged)
        self.vca_env_release_dial = buildDial(layout,6,23,'REL',0,100,self.vcaEnvReleaseChanged)
        
        self.setLayout(layout)

    def pwmWidthChanged(self):
        pass

    def pwmLfoLevelChanged(self):
        pass

    def pwmEnvLevelChanged(self):
        pass

    def pwmLfoDelayChanged(self):
        pass

    def pwmLfoRateChanged(self):
        pass

    def pwmEnvAttackChanged(self):
        pass

    def pwmEnvDecayChanged(self):
        pass

    def pwmEnvSustainChanged(self):
        pass

    def pwmEnvReleaseChanged(self):
        pass

    def dcoPwmLevelChanged(self):
        pass

    def dcoSawLevelChanged(self):
        pass

    def dcoSubLevelChanged(self):
        pass

    def dcoNoiseLevelChanged(self):
        pass

    def dcoLfoLevelChanged(self):
        pass

    def dcoLfoDelayChanged(self):
        pass

    def dcoLfoRateChanged(self):
        pass

    def hpfCutoffChanged(self):
        pass

    def vcfCutoffChanged(self):
        pass

    def vcfResonanceChanged(self):
        pass

    def vcfKeySenseChanged(self):
        pass

    def vcfVelocityChanged(self):
        pass

    def vcfLfoLevelChanged(self):
        pass

    def vcfEnvLevelChanged(self):
        pass

    def vcfKeySenseChanged(self):
        pass

    def vcfVelocityChanged(self):
        pass

    def vcfLfoLevelChanged(self):
        pass

    def vcfLfoDelayChanged(self):
        pass

    def vcfLfoRateChanged(self):
        pass

    def vcfEnvAttackChanged(self):
        pass

    def vcfEnvDecayChanged(self):
        pass

    def vcfEnvSustainChanged(self):
        pass

    def vcfEnvReleaseChanged(self):
        pass

    def vcaKeySenseChanged(self):
        pass

    def vcaVelocityChanged(self):
        pass

    def vcaLfoLevelChanged(self):
        pass

    def vcaLfoDelayChanged(self):
        pass

    def vcaLfoRateChanged(self):
        pass

    def vcaEnvAttackChanged(self):
        pass

    def vcaEnvDecayChanged(self):
        pass

    def vcaEnvSustainChanged(self):
        pass

    def vcaEnvReleaseChanged(self):
        pass
