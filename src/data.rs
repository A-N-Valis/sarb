use std::collections::VecDeque;

pub struct PriceWindow {
    buffer: VecDeque<f64>,
    capacity: usize,
}

impl PriceWindow {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity
        }
    }

    pub fn push(&mut self, price: f64) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }

        self.buffer.push_back(price);
    }

    pub fn is_ready(&self) -> bool {
        self.buffer.len() == self.capacity
    }

    pub fn to_vec(&self) -> Vec<f64> {
        self.buffer.iter().copied().collect()
    }
}