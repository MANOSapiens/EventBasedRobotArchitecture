from consts import *; from Port import PortDefinition; from StartExecution import start_execution; from Logger import Notetaker; import os, array

def main():
    # Initialize logger
    notetaker = Notetaker('/flash/log.txt')
    light.color(0, color.GREEN)

    # Define ports
    port_definitions = PortDefinition(
        l_drive_port=port.A,
        r_drive_port=port.B,
        l_tool_port=port.C,
        r_tool_port=port.D,
        colour_port=None,
    )

    paths = []

    # Glob for instruction files
    for path in os.listdir('/flash/instructions/'):
        if '.json' in path:
            paths.append('/flash/instructions/'+path)

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
    index_prev = -1
    while True:

        if button.pressed(button.LEFT):
            print('start')
            notetaker.log("Starting execution!")
            result = start_execution(paths[index], port_definitions, active_table, terminated_table, cond_table, notetaker)
            if result == 0 and index < len(paths) - 1:
                index += 1

        if button.pressed(button.RIGHT) and index < len(paths) - 1:
            index += 1
            

        #if Button.LEFT in pressed and index > 0:
        #    index -= 1
            
        if index != index_prev:
            light_matrix.write(str(index+1))
            index_prev = index

if __name__ == "__main__":
    main()