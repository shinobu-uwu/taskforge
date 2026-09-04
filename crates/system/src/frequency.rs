use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Frequency(u64);

impl Frequency {
    pub fn from_mhz(mhz: u64) -> Self {
        Self(mhz)
    }

    pub fn from_ghz(ghz: u64) -> Self {
        Self(ghz * 1000)
    }

    pub fn mhz(&self) -> u64 {
        self.0
    }

    pub fn ghz(&self) -> u64 {
        self.0 / 1000
    }

    pub fn ghz_f64(&self) -> f64 {
        self.0 as f64 / 1000.0
    }
}

impl Display for Frequency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
