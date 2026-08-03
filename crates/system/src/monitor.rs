use circular_buffer::FixedCircularBuffer;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, RefreshKind, System};

const HISTORY_LEN: usize = 1024;

#[derive(Debug)]
pub struct SystemMonitor {
    pub system: System,
    pub cpu_history: FixedCircularBuffer<f32, HISTORY_LEN>,
    pub memory_history: FixedCircularBuffer<u64, HISTORY_LEN>,
}

impl SystemMonitor {
    pub fn new(system: System) -> Self {
        Self {
            system,
            cpu_history: FixedCircularBuffer::new(),
            memory_history: FixedCircularBuffer::new(),
        }
    }

    pub fn poll(&mut self) {
        self.system.refresh_specifics(Self::refresh_kind());
        self.cpu_history.push_back(self.system.global_cpu_usage());
        self.memory_history.push_back(self.system.used_memory());
    }

    fn refresh_kind() -> RefreshKind {
        RefreshKind::nothing()
            .with_processes(ProcessRefreshKind::everything())
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        let system = System::new_with_specifics(Self::refresh_kind());
        Self::new(system)
    }
}
