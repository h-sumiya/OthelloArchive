use std::time::Instant;

const LiMIT_TIME: u128 = 140;
pub struct TimeManager {
    start: Instant,
    last: u128,
}

impl TimeManager {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            last: 0,
        }
    }

    pub fn lap(&mut self) -> Result<(), ()> {
        let elapsed = self.start.elapsed().as_millis();
        eprintln!("elapsed: {}", elapsed);
        let diff = elapsed - self.last;
        if elapsed + diff > LiMIT_TIME {
            return Err(());
        } else {
            self.last = elapsed;
            return Ok(());
        }
    }
}
