use std::{collections::HashMap, time::Duration};

use sysinfo::{
    CpuRefreshKind, Disks, MemoryRefreshKind, Pid, ProcessRefreshKind, RefreshKind, System,
    ThreadKind,
};

use crate::{
    disk::{DiskSnapshot, DiskUsage},
    memory::Memory,
};

#[derive(Debug)]
pub struct SystemMonitor {
    system: System,
    disks: Disks,
}

#[derive(Debug, Clone)]
pub struct SystemSnapshot {
    pub processes: HashMap<Pid, ProcessSnapshot>,
    pub cpu_usage: f32,
    pub memory_usage: Memory,
    pub total_memory: Memory,
    pub uptime: Duration,
    pub disks: Vec<DiskSnapshot>,
}

#[derive(Debug, Clone)]
pub struct ProcessSnapshot {
    pub pid: Pid,
    pub parent: Option<Pid>,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: Memory,
    pub thread_kind: Option<ThreadKind>,
}

impl SystemMonitor {
    pub fn new(system: System, disks: Disks) -> Self {
        Self { system, disks }
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
                            memory: Memory::from_bytes(proc.memory()),
                            thread_kind: proc.thread_kind(),
                        },
                    )
                })
                .collect(),
            cpu_usage: self.system.global_cpu_usage(),
            memory_usage: Memory::from_bytes(self.system.used_memory()),
            total_memory: Memory::from_bytes(self.system.total_memory()),
            uptime: Duration::from_secs(System::uptime()),
            disks: self
                .disks
                .iter()
                .map(|disk| DiskSnapshot {
                    name: disk.name().to_string_lossy().into_owned(),
                    usage: DiskUsage {
                        written_bytes: disk.usage().written_bytes,
                        read_bytes: disk.usage().read_bytes,
                    },
                })
                .collect(),
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_specifics(Self::refresh_kind());
        self.disks.refresh(true);
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
        let disks = Disks::new_with_refreshed_list();
        Self::new(system, disks)
    }
}
