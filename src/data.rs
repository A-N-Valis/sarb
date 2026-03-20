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

    pub fn fill_slice(&self, out: &mut Vec<f64>) {
        out.clear();
        let (front, back) = self.buffer.as_slices();
        out.extend_from_slice(front);
        out.extend_from_slice(back);
    }
}