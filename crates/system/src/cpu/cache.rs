use std::fmt::{Display, Formatter};

use crate::memory::Memory;

#[derive(Debug, Clone, Default)]
pub struct Cache {
    pub level: Level,
    pub size: Memory,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Level(usize);

impl Level {
    pub fn new(level: usize) -> Self {
        Self(level)
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
