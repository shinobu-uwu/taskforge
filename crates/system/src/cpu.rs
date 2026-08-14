#[derive(Debug, Clone)]
pub struct CpuUsage {
    pub total: f32,
    pub cpus: Vec<f32>,
}
