pub struct Timers {
    pub last_time: std::time::Instant,
    pub next_update: std::time::Duration,
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Timers {
    pub fn new() -> Self {
        Timers {
            last_time: std::time::Instant::now(),
            next_update: std::time::Duration::from_millis(17),
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn update(&mut self) {
        let now = std::time::Instant::now();
        if now - self.last_time > self.next_update {
            self.last_time = now;
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
        }
    }
}
