use std::{
    ops::{Deref, DerefMut},
    time::Instant,
};

use circular_buffer::FixedCircularBuffer;
use system::disk::DiskUsage;

pub const HISTORY_LEN: usize = 128;

#[derive(Debug, Clone, Default)]
pub struct History<T>(FixedCircularBuffer<T, HISTORY_LEN>);

#[derive(Debug, Clone, Default)]
pub struct NetworkHistory {
    pub received: History<u64>,
    pub sent: History<u64>,
}

#[derive(Debug, Clone)]
pub struct DiskHistory {
    pub name: String,
    pub usage: History<TimedSample<DiskUsage>>,
}

#[derive(Debug, Clone)]
pub struct TimedSample<T> {
    pub captured_at: Instant,
    pub sample: T,
}

impl<T> TimedSample<T> {
    pub fn new(captured_at: Instant, sample: T) -> Self {
        Self {
            captured_at,
            sample,
        }
    }
}

impl DiskHistory {
    pub fn new(name: String, captured_at: Instant, usage: DiskUsage) -> Self {
        let mut history = History::new();
        history.push_back(TimedSample::new(captured_at, usage));

        Self {
            name,
            usage: history,
        }
    }
}

impl<T> History<T> {
    pub fn new() -> Self {
        Self(FixedCircularBuffer::new())
    }
}

impl<T> Deref for History<T> {
    type Target = FixedCircularBuffer<T, HISTORY_LEN>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for History<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
