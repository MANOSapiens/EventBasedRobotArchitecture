// Local modules

use crate::Events::PID;



pub fn ComputePID(value: f32, target: &mut f32, delta_t: f32, pid: &mut PID) -> f32{
    let error: f32 = *target - value;
    
    if pid.prev_e < 0.0 {
        pid.prev_e = error;
    }

    pid.sum_i += pid.i * error * delta_t*1000.0;
    if pid.sum_i > pid.max_i { 
        pid.sum_i = pid.max_i;
    } else if pid.sum_i < -pid.max_i {
        pid.sum_i = -pid.max_i;
    }

    let result: f32 = pid.p * error + (pid.d * (error - pid.prev_e)) / (delta_t*1000.0) + pid.sum_i;
    
    pid.prev_e = error;
    result
}

pub fn ComputePIDGyro(value: f32, target: &mut f32, delta_s: f32, pid: &mut PID) -> f32{
    let error: f32 = *target - value;
    
    if pid.prev_e < 0.0 {
        pid.prev_e = error;
    } 

    pid.sum_i += pid.i * error.to_radians().sin() * delta_s;
    if pid.sum_i > pid.max_i {
        pid.sum_i = pid.max_i;
    } else if pid.sum_i < -pid.max_i {
        pid.sum_i = -pid.max_i;
    }

    let result: f32 = pid.p * error + pid.d * (error - pid.prev_e) + pid.sum_i;
    
    pid.prev_e = error;
    result
}