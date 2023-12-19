// Import crates
extern crate ev3dev_lang_rust;



use std::time::{Instant};

use crate::ProcessLoop::SensorActuatorValues;





pub struct RoundSummary {
    pub wall_time: u64,
    pub max_loop_time: f32,
    pub mean_loop_time: f32,
    pub total_travelled_distance: i32,
    pub loop_count: u64,
    pub mean_f: u32
}

pub fn Check(
    round_summary: &mut RoundSummary,
    _sensor_act_values: &SensorActuatorValues,
    _round_timeout: &f32
) {
    round_summary.loop_count += 1;
    let elapsed: f32 = _sensor_act_values.currentTime - _sensor_act_values.timePrev;

    /* if DEBUG && *round_timeout != -1.0 {
        if (elapsed as f32) > *round_timeout {
            error!("Round timeout because it already took {}, more than the timeout of {}!", elapsed, round_timeout);
        }
    } */

    // Take down maximum loop time
    if round_summary.max_loop_time < elapsed {
        round_summary.max_loop_time = elapsed;
    }
    
    // Check sensor values
}
