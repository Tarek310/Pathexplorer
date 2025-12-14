use core::fmt;
use std::{collections::VecDeque, usize};

pub struct StringRingBuffer {
    buffer: VecDeque<String>,
    capacity: usize,
    error_count: usize,
}

impl StringRingBuffer {
    pub fn with_capacity(capacity: usize) -> StringRingBuffer {
        StringRingBuffer {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            error_count: 0,
        }
    }

    pub fn push(&mut self, s: String) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }
        self.error_count += 1;
        self.buffer
            .push_back(format!("[ERROR {}]: {}", self.error_count, s));
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}
impl fmt::Display for StringRingBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.buffer
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

