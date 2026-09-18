use std::fmt::{Display, Formatter};

const BYTES_PER_KIBIBYTE: f64 = 1024.0;
const BYTES_PER_MEBIBYTE: f64 = BYTES_PER_KIBIBYTE * 1024.0;
const BYTES_PER_GIBIBYTE: f64 = BYTES_PER_MEBIBYTE * 1024.0;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Memory(u64);

impl Memory {
    pub const ZERO: Self = Self(0);

    pub const fn from_bytes(bytes: u64) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(self) -> u64 {
        self.0
    }

    pub fn as_kib_f64(self) -> f64 {
        self.0 as f64 / BYTES_PER_KIBIBYTE
    }

    pub fn as_mib_f64(self) -> f64 {
        self.0 as f64 / BYTES_PER_MEBIBYTE
    }

    pub fn as_gib_f64(self) -> f64 {
        self.0 as f64 / BYTES_PER_GIBIBYTE
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        const UNITS: [&str; 7] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];

        let mut divisor = 1_u64;
        let mut unit = 0;
        while unit + 1 < UNITS.len() && self.0 / divisor >= 1024 {
            divisor *= 1024;
            unit += 1;
        }

        write!(f, "{}{}", self.0 as f64 / divisor as f64, UNITS[unit])
    }
}

impl std::ops::AddAssign for Memory {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;

    #[test]
    fn converts_binary_units() {
        let memory = Memory::from_bytes(3 * 1024_u64.pow(3));

        assert_eq!(memory.as_bytes(), 3 * 1024_u64.pow(3));
        assert_eq!(memory.as_kib_f64(), 3.0 * 1024.0 * 1024.0);
        assert_eq!(memory.as_mib_f64(), 3.0 * 1024.0);
        assert_eq!(memory.as_gib_f64(), 3.0);
    }

    #[test]
    fn displays_binary_units() {
        for (bytes, expected) in [
            (0, "0B"),
            (1, "1B"),
            (1023, "1023B"),
            (1024, "1KiB"),
            (16 * 1024, "16KiB"),
            (1024 * 1024, "1MiB"),
            (4096 * 1024, "4MiB"),
            (3 * 1024_u64.pow(3) / 2, "1.5GiB"),
            (1024_u64.pow(4), "1TiB"),
            (1024_u64.pow(5), "1PiB"),
            (1024_u64.pow(6), "1EiB"),
        ] {
            assert_eq!(Memory::from_bytes(bytes).to_string(), expected);
        }
    }
}
