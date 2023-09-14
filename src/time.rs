use std::time::Instant;

const LIMIT_TIME: u128 = 150;
pub struct TimeManager {
    start: Instant,
    last: u128,
    max_diff: u128,
}

impl TimeManager {
    pub const fn temp() -> Self {
        unsafe { std::mem::transmute([0u8; 48]) }
    }

    #[allow(dead_code)]
    pub fn init(&mut self) {
        let mut start = Instant::now();
        std::mem::swap(&mut self.start, &mut start);
        std::mem::forget(start);
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
        self.last = 0;
        self.max_diff = 0;
    }

    pub fn lap(&mut self) -> Result<(), ()> {
        let elapsed = self.start.elapsed().as_millis();
        let mut diff = elapsed - self.last;
        if diff > self.max_diff {
            self.max_diff = diff;
            diff = self.max_diff;
        }
        if elapsed + (diff * 3) > LIMIT_TIME {
            return Err(());
        } else {
            self.last = elapsed;
            return Ok(());
        }
    }

    #[target_feature(enable = "avx2")]
    pub unsafe fn check(&self) -> Result<(), ()> {
        let elapsed = self.start.elapsed().as_millis();
        let diff = elapsed - self.last;
        if diff * 3 > (LIMIT_TIME - self.last) {
            return Err(());
        } else {
            return Ok(());
        }
    }

    pub fn next(&mut self) -> Result<(), ()> {
        let elapsed = self.start.elapsed().as_millis();
        self.last = elapsed;
        if elapsed * 6 < LIMIT_TIME {
            eprintln!("next"); //python:debug
            return Ok(());
        } else {
            eprintln!("break next"); //python:debug
            return Err(());
        }
    }

    pub fn finish(&self) {
        let elapsed = self.start.elapsed().as_millis();
        eprintln!("finish: {}", elapsed); //python:debug
    }
}
