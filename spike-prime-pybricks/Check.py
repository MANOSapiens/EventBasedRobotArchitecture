from consts import *


class RoundSummary:
    def __init__(self):
        self.wall_time = 0
        self.mean_loop_time = 0.0
        self.mean_f = 0  # Frequency in Hz
        self.loop_count = 0
        self.max_loop_time = 0.0
        self.total_travelled_distance = 0


def check(round_summary, sensor_act_values):
    round_summary.loop_count += 1
    if sensor_act_values.timePrev == -1.0:
        sensor_act_values.timePrev = sensor_act_values.currentTime
        
    elapsed = sensor_act_values.currentTime - sensor_act_values.timePrev

    # Take down maximum loop time
    if round_summary.max_loop_time < elapsed:
        round_summary.max_loop_time = elapsed

    # Additional checks for sensor values can be added here
