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
}
