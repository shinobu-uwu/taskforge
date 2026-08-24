#[derive(Debug, Clone)]
pub struct CpuUsage {
    pub total: f32,
    pub frequency: u64,
    pub cpus: Vec<f32>,
}
