// ADJUST THESE PARAMETERS TO INCREASE PERFORMANCE
pub const DEBUG: bool = true;
pub const SPARSE_SENSOR_READING: bool = true;

pub const LDRIVEENC: i8 = 0;
pub const RDRIVEENC: i8 = 1;
pub const LTOOLENC: i8 = 2;
pub const RTOOLENC: i8 = 3;
pub const COLOURSENS: i8 = 4;
pub const GYRO: i8 = 5;
pub const LDRIVEPOW: i8 = 6;
pub const RDRIVEPOW: i8 = 7;
pub const LTOOLPOW: i8 = 8;
pub const RTOOLPOW: i8 = 9;
pub const LDRIVECOR: i8 = 10;
pub const RDRIVECOR: i8 = 11;
pub const LTOOLCOR: i8 = 12;
pub const RTOOLCOR: i8 = 13;
pub const CENTERBUTTON: i8 = 18;
pub const DRIVEENC: i8 = 19;
pub const DRIVESPEED: i8 = 20;
pub const LDRIVESPEED: i8 = 21;
pub const RDRIVESPEED: i8 = 22;
pub const LTOOLSPEED: i8 = 23;
pub const RTOOLSPEED: i8 = 24;



pub const EVENT_NONE: i8 = 0;
pub const EVENT_PLACEHOLDER: i8 = 1;
pub const EVENT_SENSORVALUE: i8 = 2;
pub const EVENT_MOTORSPEEDCONTROL: i8 = 3;
pub const EVENT_PIDGYRO: i8 = 4;
pub const EVENT_PIDLINE: i8 = 5;
pub const EVNET_PIDHOLD: i8 = 6;
pub const EVENT_TIMER: i8 = 7;
pub const EVENT_COMPUTEMOTORSTALL: i8 = 8;