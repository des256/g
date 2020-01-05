# SOCIAL0 - social battle system exploration
# scenario: reception desk: get access to 3rd floor of hotel

import sys
import numpy
#import pye

class Business:

    def __init__(self):

        # business state
        self.b_state = numpy.array([0.0,0.0,0.0])


class Action:

    def __init__(self):

        # business action
        self.b_action = numpy.array([0.0,0.0,0.0])


class Character:

    def __init__(self):

        # emotional state
        self.e = numpy.array([0.0,0.0,0.0,0.0])

        # business influence
        self.b_influence = numpy.array([
            [1.0,0.0,0.0],
            [0.0,1.0,0.0],
            [0.0,0.0,1.0]
        ])

        # business influence bias
        self.b_bias = numpy.array([0.0,0.0,0.0])

        # emotion influence
        self.e_influence = numpy.array([
            [1.0,0.0,0.0,0.0],
            [0.0,1.0,0.0,0.0],
            [0.0,0.0,1.0,0.0],
            [0.0,0.0,0.0,1.0]
        ])

        # emotion influence bias
        self.e_bias = numpy.array([0.0,0.0,0.0,0.0])

        # business state change appraisal
        self.b_appraisal = numpy.array([
            [0.0,0.0,0.0,0.0],
            [0.0,0.0,0.0,0.0],
            [0.0,0.0,0.0,0.0]
        ])

        # ability to read emotions
        self.e_quotient = numpy.array([
            [1.0,0.0,0.0],
            [0.0,1.0,0.0],
            [0.0,0.0,1.0]
        ])


# access possible if: she likes you, you have credentials, your room is at the 3rd floor, you force her, you trick her, 

if __name__ == '__main__':

