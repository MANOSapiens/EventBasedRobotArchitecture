import csv

class Notetaker:
    def __init__(self, log_file):
        self.log_file = log_file
        self.file = open(log_file, 'a+')

    def log(self, message):
        print('[INFO] {}\n'.format(message))
        self.file.write('[INFO] {}\n'.format(message))
        self.file.flush()

    def warn(self, message):
        self.file.write('[WARNING] {}\n'.format(message))
        self.file.flush()

    def error(self, message):
        self.file.write('[ERROR] {}\n'.format(message))
        self.file.flush()


def init_logger(log_file):
    file = open(log_file, 'w')
    return file


def log_header_csv(writer):
    headers = [
        "currentTime",
        "lDriveMotorEnc",
        "rDriveMotorEnc",
        "lToolMotorEnc",
        "rToolMotorEnc",
        "gyroAngValue",
        "lDriveMotorPow",
        "rDriveMotorPow",
        "lToolMotorPow",
        "rToolMotorPow",
        "lDriveMotorCor",
        "rDriveMotorCor",
        "lDriveSpeed",
        "rDriveSpeed",
    ]
    writer.write(','.join(headers)+'\n')
    # Ensure the writer is flushed to write headers immediately
    writer.flush()


def log_csv(writer, sensor_act_values):
    row = [
        sensor_act_values[TIME],
        sensor_act_values[LDRIVEENC],
        sensor_act_values[RDRIVEENC],
        sensor_act_values[LTOOLENC],
        sensor_act_values[RTOOLENC],
        sensor_act_values[GYRO],
        sensor_act_values[LDRIVEPOW],
        sensor_act_values[RDRIVEPOW],
        sensor_act_values[LTOOLPOW],
        sensor_act_values[RTOOLPOW],
        sensor_act_values[LDRIVECOR],
        sensor_act_values[RDRIVECOR],
        sensor_act_values[LDRIVESPEED],
        sensor_act_values[RDRIVESPEED],
    ]
    writer.write(','.join([str(i) for i in row])+'\n')
    # Ensure the writer is flushed to save the data immediately
    writer.flush()