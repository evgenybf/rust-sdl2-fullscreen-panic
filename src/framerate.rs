use sdl2::timer;

pub struct FPSManager {
    frame_delay_ms: u32,
    start_time_ms: u32,
    prev_start_time_ms: u32,
}

impl FPSManager {
    pub fn new(frame_rate: u32) -> FPSManager {
        FPSManager {
            frame_delay_ms: 1000 / frame_rate, 
            start_time_ms: 0,
            prev_start_time_ms: 0,
        }
    }
    
    pub fn delay(&mut self) {
        let iter_time = timer::get_ticks() - self.start_time_ms;

        let delay_time = if self.frame_delay_ms  > iter_time { 
            (self.frame_delay_ms  - iter_time)
        } else { 
            0
        };
        
        timer::delay(delay_time);
        
        self.prev_start_time_ms = self.start_time_ms;
        self.start_time_ms = timer::get_ticks();
    }
    
    pub fn calc_elapsed_time(&self) -> u32 {
        let current_time_ms = timer::get_ticks();
        current_time_ms - self.start_time_ms
    }
    
    pub fn get_fps(&self) -> f64 {
        let seconds_per_frame = (self.start_time_ms - self.prev_start_time_ms) as f64 / 1000.0;
        1.0 / seconds_per_frame
    }
    
}

