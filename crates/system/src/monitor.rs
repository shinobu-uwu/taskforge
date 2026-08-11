use std::{collections::HashMap, time::Duration};

use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, Pid, ProcessRefreshKind, RefreshKind, System, ThreadKind,
};

#[derive(Debug)]
pub struct SystemMonitor {
    pub system: System,
}

#[derive(Debug, Clone)]
pub struct SystemSnapshot {
    pub processes: HashMap<Pid, ProcessSnapshot>,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub total_memory: u64,
    pub uptime: Duration,
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
                .iter()
                .map(|(pid, proc)| {
                    (
                        *pid,
                        ProcessSnapshot {
                            pid: proc.pid(),
                            parent: proc.parent(),
                            name: proc.name().to_string_lossy().into_owned(),
                            cpu_usage: proc.cpu_usage(),
                            memory: proc.memory(),
                            thread_kind: proc.thread_kind(),
                        },
                    )
                })
                .collect(),
            cpu_usage: self.system.global_cpu_usage(),
            memory_usage: self.system.used_memory(),
            total_memory: self.system.total_memory(),
            uptime: Duration::from_secs(System::uptime()),
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_specifics(Self::refresh_kind());
    }

    pub fn kill_process(&mut self, pid: Pid) -> bool {
        self.system.processes().get(&pid).is_some_and(|p| p.kill())
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
