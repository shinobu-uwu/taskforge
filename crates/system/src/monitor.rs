use std::time::{Duration, Instant};

use sysinfo::{
    CpuRefreshKind, Disks, MemoryRefreshKind, ProcessRefreshKind, RefreshKind, System, UpdateKind,
    Users,
};

use crate::{
    cpu::{CpuInfo, CpuUsage},
    disk::{DiskSnapshot, DiskUsage},
    memory::Memory,
    native::{Backend, NativeBackend},
    process::Pid,
    snapshot::{ProcessSnapshot, SystemSnapshot},
};

#[derive(Debug)]
pub struct SystemMonitor {
    system: System,
    disks: Disks,
    users: Users,
    backend: NativeBackend,
}

impl SystemMonitor {
    pub fn new(system: System, disks: Disks, users: Users, backend: NativeBackend) -> Self {
        Self {
            system,
            disks,
            users,
            backend,
        }
    }

    pub fn cpu_info(&self) -> CpuInfo {
        self.backend.cpu_info()
    }

    pub fn snapshot(&self) -> SystemSnapshot {
        SystemSnapshot {
            thread_count: self.backend.thread_count(),
            logical_processor_count: self.backend.logical_processor_count(),
            handle_count: self.backend.handle_count(),
            captured_at: Instant::now(),
            processes: self
                .system
                .processes()
                .iter()
                .map(|(pid, proc)| {
                    (
                        Pid::from(*pid),
                        ProcessSnapshot {
                            pid: proc.pid().into(),
                            parent: proc.parent().map(Into::into),
                            name: proc.name().to_string_lossy().into_owned(),
                            cpu_usage: proc.cpu_usage(),
                            memory: Memory::from_bytes(proc.memory()),
                            thread_kind: proc.thread_kind().map(Into::into),
                            username: proc.effective_user_id().map(|id| {
                                self.users
                                    .get_user_by_id(id)
                                    .expect("User not found")
                                    .name()
                                    .to_string()
                            }),
                        },
                    )
                })
                .collect(),
            cpu_usage: CpuUsage {
                total: self.system.global_cpu_usage(),
                cpus: self.system.cpus().iter().map(|c| c.cpu_usage()).collect(),
                frequency: self
                    .system
                    .cpus()
                    .first()
                    .expect("No CPU core to collect frequency from")
                    .frequency()
                    .into(),
            },
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
        self.users.refresh();
    }

    pub fn kill_process(&mut self, pid: Pid) -> bool {
        self.system
            .processes()
            .get(&sysinfo::Pid::from_u32(pid.as_u32()))
            .is_some_and(|p| p.kill())
    }

    fn refresh_kind() -> RefreshKind {
        RefreshKind::nothing()
            .with_processes(
                ProcessRefreshKind::nothing()
                    .with_cpu()
                    .with_memory()
                    .with_user(UpdateKind::Always)
                    .without_tasks(),
            )
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        let system = System::new_with_specifics(Self::refresh_kind());
        let disks = Disks::new_with_refreshed_list();
        let users = Users::new_with_refreshed_list();
        let backend = NativeBackend::new();
        Self::new(system, disks, users, backend)
    }
}
