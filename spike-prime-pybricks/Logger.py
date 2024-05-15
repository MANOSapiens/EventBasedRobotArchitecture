import csv


class Notetaker:
    def __init__(self, log_file):
        self.log_file = log_file
        self.file = open(log_file, 'a+')

    def log(self, message):
        self.file.write(f'[INFO] {message}\n')
        self.file.flush()

    def warn(self, message):
        self.file.write(f'[WARNING] {message}\n')
        self.file.flush()

    def error(self, message):
        self.file.write(f'[ERROR] {message}\n')
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
        sensor_act_values.currentTime,
        sensor_act_values.lDriveMotorEnc,
        sensor_act_values.rDriveMotorEnc,
        sensor_act_values.lToolMotorEnc,
        sensor_act_values.rToolMotorEnc,
        sensor_act_values.gyroAngValue,
        sensor_act_values.lDriveMotorPow,
        sensor_act_values.rDriveMotorPow,
        sensor_act_values.lToolMotorPow,
        sensor_act_values.rToolMotorPow,
        sensor_act_values.lDriveMotorCor,
        sensor_act_values.rDriveMotorCor,
        sensor_act_values.lDriveMotorSpeed,
        sensor_act_values.rDriveMotorSpeed,
    ]
    writer.write(','.join([str(i) for i in  row])+'\n')
    # Ensure the writer is flushed to save the data immediately
    writer.flush()

