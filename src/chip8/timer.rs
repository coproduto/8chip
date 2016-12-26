pub struct Timers {
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl super::Chip8 {
    fn timer_cycle(&mut self) {
        //might want to add extra logic here
        self.timers.timer_cycle();
    }
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn timer_cycle(&mut self) {
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

    pub fn sound_buzzer(&self) {
        //dummy implementation
        println!("Bzzzzzz!");
    }
}
