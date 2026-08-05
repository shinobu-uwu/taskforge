use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, Pid, ProcessRefreshKind, RefreshKind, System, ThreadKind,
};

#[derive(Debug)]
pub struct SystemMonitor {
    pub system: System,
}

#[derive(Debug, Clone)]
pub struct SystemSnapshot {
    pub processes: Vec<ProcessSnapshot>,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub total_memory: u64,
}

#[derive(Debug, Clone)]
pub struct ProcessSnapshot {
    pub pid: Pid,
    pub parent: Option<Pid>,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub thread_kind: Option<ThreadKind>,
}

impl SystemMonitor {
    pub fn new(system: System) -> Self {
        Self { system }
    }

    pub fn snapshot(&self) -> SystemSnapshot {
        SystemSnapshot {
            processes: self
                .system
                .processes()
                .values()
                .map(|p| ProcessSnapshot {
                    pid: p.pid(),
                    parent: p.parent(),
                    name: p.name().to_string_lossy().into_owned(),
                    cpu_usage: p.cpu_usage(),
                    memory: p.memory(),
                    thread_kind: p.thread_kind(),
                })
                .collect(),
            cpu_usage: self.system.global_cpu_usage(),
            memory_usage: self.system.used_memory(),
            total_memory: self.system.total_memory(),
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_specifics(Self::refresh_kind());
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
