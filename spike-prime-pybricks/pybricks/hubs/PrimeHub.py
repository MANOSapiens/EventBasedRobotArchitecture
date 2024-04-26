class IMU:
    def __init__(self):
        pass
    
    def heading(self):
        return 0
    
    def reset_heading(self, angle):
        pass


class Buttons:
    def __init__(self):
        pass
    
    def pressed(self):
        return []

class PrimeHub:
    def __init__(self):
        self.imu = IMU()
        self.buttons = Buttons()