// Xorshift
// use std::time::SystemTime;

pub struct Rand {
    seed: u64,
}

impl Rand {
    pub fn new() -> Rand {
        //let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("can not get system time").as_secs();
        let seed = 100;
        Rand {
            seed
        }
    }

    fn next_seed(&mut self) -> u64 {
        let prev = self.seed;
        self.seed = self.seed ^ (self.seed << 13);
        self.seed = self.seed ^ (self.seed >> 17);
        self.seed = self.seed ^ (self.seed << 5);
        prev
    }

    pub fn next_f64(&mut self) -> f64 {
        (self.next_seed() as f64) / (std::u64::MAX as f64)
    }
}
