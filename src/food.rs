#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Food {
    pub count: u64
}

impl Food {
    pub fn new(count: u64) -> Food {
        Food { count }
    }

    pub fn add(&mut self, count: u64) {
        self.count += count;
    }

    pub fn reduce(&mut self, count: u64) {
        self.count -= count;
    }
}