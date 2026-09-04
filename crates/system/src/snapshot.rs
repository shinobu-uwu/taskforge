use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{
    cpu::CpuUsage,
    disk::DiskSnapshot,
    memory::Memory,
    process::{Pid, ThreadKind},
};

#[derive(Debug, Clone)]
pub struct SystemSnapshot {
    pub captured_at: Instant,
    pub processes: HashMap<Pid, ProcessSnapshot>,
    pub cpu_usage: CpuUsage,
    pub memory_usage: Memory,
    pub total_memory: Memory,
    pub uptime: Duration,
    pub disks: Vec<DiskSnapshot>,
    pub core_count: Option<usize>,
    pub logical_processor_count: Option<usize>,
    pub descriptors_count: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct ProcessSnapshot {
    pub pid: Pid,
    pub parent: Option<Pid>,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: Memory,
    pub thread_kind: Option<ThreadKind>,
    pub username: Option<String>,
}
