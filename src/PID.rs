// Local modules

use crate::Events::PID;



pub fn ComputePID(value: f32, target: &mut f32, pid: &mut PID) -> f32{
    let error: f32 = *target - value;
    
    if pid.prev_e < 0.0 {
        pid.prev_e = error;
    }

    pid.sum_i += pid.i * error;
    if pid.sum_i > pid.max_i {
        pid.sum_i = pid.max_i;
    }

    let result: f32 = pid.p * error + pid.d * (error - pid.prev_e) + pid.sum_i;
    
    pid.prev_e = error;
    result
}