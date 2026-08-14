#[derive(Debug, Clone)]
pub struct DiskSnapshot {
    pub name: String,
    pub usage: DiskUsage,
}

#[derive(Debug, Clone, Copy)]
pub struct DiskUsage {
    pub written_bytes: u64,
    pub read_bytes: u64,
}
