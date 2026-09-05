use crate::frequency::Frequency;

#[derive(Debug, Clone)]
pub struct CpuUsage {
    pub total: f32,
    pub frequency: Frequency,
    pub cpus: Vec<f32>,
}

#[derive(Debug, Clone, Default)]
pub struct CpuInfo {
    pub name: Option<String>,
    pub core_count: Option<usize>,
    pub socket_count: Option<usize>,
    pub base_frequency: Option<Frequency>,
    pub caches: Option<Vec<usize>>,
    pub virtualization_enabled: Option<bool>,
}
