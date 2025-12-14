use std::{collections::VecDeque, usize};

pub struct StringRingBuffer {
    buffer: VecDeque<String>,
    capacity: usize,
}

impl StringRingBuffer {
    pub fn with_capacity(capacity: usize) -> StringRingBuffer {
        StringRingBuffer {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, s: String) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(s);
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn to_string(&mut self) -> String {
        self.buffer
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

