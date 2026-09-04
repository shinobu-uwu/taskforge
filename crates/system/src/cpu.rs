use crate::frequency::Frequency;

#[derive(Debug, Clone)]
pub struct CpuUsage {
    pub total: f32,
    pub frequency: u64,
    pub cpus: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub name: String,
    pub core_count: usize,
    pub sockets: usize,
    pub base_frequency: Frequency,
    pub caches: Vec<usize>,
    pub virtualization_enabled: bool,
}
