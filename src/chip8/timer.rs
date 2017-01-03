pub struct Timers {
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn cycle(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
        if self.sound_timer == 0 {
            self.sound_buzzer();
        }
    }

    fn sound_buzzer(&self) {
        //dummy implementation
        println!("Bzzzzzz!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    quickcheck!{
        fn test_new_timers() -> bool {
            let timers = Timers::new();

            timers.delay_timer == 0 && timers.sound_timer == 0
        }

        fn test_stopped_timers() -> bool {
            let mut timers = Timers::new();

            timers.delay_timer = 0;
            timers.sound_timer = 0;

            timers.cycle();

            timers.delay_timer == 0 && timers.sound_timer == 0
        }

        fn test_cycling_timers(val: u8) -> bool {
            let mut timers = Timers::new();

            timers.delay_timer = val;
            timers.sound_timer = val;
            timers.cycle();

            if(val > 0) {
                timers.delay_timer == val - 1 && timers.sound_timer == val - 1
            } else {
                timers.delay_timer == 0 && timers.sound_timer == 0
            }
        }
    }
}
