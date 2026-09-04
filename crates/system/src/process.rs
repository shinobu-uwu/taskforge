use std::fmt;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Pid(u32);

impl From<usize> for Pid {
    fn from(v: usize) -> Self {
        Self(v as u32)
    }
}

impl From<sysinfo::Pid> for Pid {
    fn from(v: sysinfo::Pid) -> Self {
        Self(v.as_u32())
    }
}

impl From<Pid> for usize {
    fn from(v: Pid) -> Self {
        v.0 as _
    }
}

impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Pid {
    pub fn as_u32(self) -> u32 {
        self.0 as _
    }

    pub fn from_u32(v: u32) -> Self {
        Self(v as _)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThreadKind {
    Kernel,
    Userland,
}

impl From<sysinfo::ThreadKind> for ThreadKind {
    fn from(value: sysinfo::ThreadKind) -> Self {
        match value {
            sysinfo::ThreadKind::Kernel => ThreadKind::Kernel,
            sysinfo::ThreadKind::Userland => ThreadKind::Userland,
        }
    }
}
