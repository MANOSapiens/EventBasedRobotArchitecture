from consts import *
from Port import PortDefinition
from StartExecution import start_execution
from Logger import Notetaker
from pybricks.parameters import Port
from pybricks.hubs import PrimeHub
from pybricks.parameters import Color, Button

import glob
import os
import time
import array

def main():
    # Initialize logger
    notetaker = Notetaker('log.txt')

    # Define ports
    hub = PrimeHub()
    hub.light.on(Color.GREEN)

    port_definitions = PortDefinition(
        l_drive_port=Port.A,
        r_drive_port=Port.B,
        l_tool_port=Port.C,
        r_tool_port=Port.D,
        colour_port=Port.E,
    )

    paths = []
    filenames = []

    # Glob for instruction files
    for path in glob.glob("instructions/*.json"):
        paths.append(path)
        filename = os.path.splitext(os.path.basename(path))[0]
        filenames.append(filename)

    # Read boolean table lengths
    with open(TABLELENGTHSFILE, 'r') as file:
        contents = file.read()
    boolean_table_lengths = contents.split(";")

    # Prepare boolean tables
    active_table = array.array('b', [False] * (int(boolean_table_lengths[0]) + 1))
    active_table[0] = True
    terminated_table = array.array('b', [False] * (int(boolean_table_lengths[1]) + 1))
    cond_table = array.array('b', [False] * (int(boolean_table_lengths[1]) + int(boolean_table_lengths[2])))

    index = 0
    while True:
        pressed = hub.buttons.pressed()
        if Button.BLUETOOTH in pressed:
            break

        if Button.CENTER in pressed:
            notetaker.log("Starting execution!")
            result = start_execution(hub, paths[index], port_definitions, active_table, terminated_table, cond_table, notetaker)
            if result == 0 and index < len(paths) - 1:
                index += 1

        if Button.RIGHT in pressed and index < len(paths) - 1:
            index += 1
            time.sleep(0.250)

        if Button.LEFT in pressed and index > 0:
            index -= 1
            time.sleep(0.250)

        hub.display.number(index+1)

if __name__ == "__main__":
    main()
