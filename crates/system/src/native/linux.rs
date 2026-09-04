use std::collections::HashSet;

use sysinfo::System;

use crate::{cpu::CpuInfo, frequency::Frequency};

const ONLINE_CPUS_PATH: &str = "/sys/devices/system/cpu/online";

#[derive(Debug, Clone)]
pub struct LinuxBackend;

impl super::Backend for LinuxBackend {
    fn new() -> Self {
        Self
    }

    fn core_count(&self) -> Option<usize> {
        System::physical_core_count()
    }

    fn logical_processor_count(&self) -> Option<usize> {
        let cpu_list = std::fs::read_to_string(ONLINE_CPUS_PATH).ok()?;
        parse_cpu_list(&cpu_list)
    }

    fn handle_count(&self) -> Option<usize> {
        std::fs::read_to_string("/proc/sys/fs/file-nr")
            .ok()?
            .split_whitespace()
            .next()?
            .parse()
            .ok()
    }

    fn cpu_info(&self) -> Option<CpuInfo> {
        let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").ok()?;
        parse_cpuinfo(&cpuinfo)
    }
}

fn parse_cpuinfo(cpuinfo: &str) -> Option<CpuInfo> {
    let name = cpuinfo
        .lines()
        .find(|l| l.starts_with("model name"))?
        .split_once(':')?
        .1
        .trim()
        .to_string();
    let core_count = System::physical_core_count()?;
    let sockets: HashSet<usize> = cpuinfo
        .lines()
        .filter_map(|l| {
            if !l.starts_with("physical id") {
                None
            } else {
                l.split_once(':')?.1.trim().parse().ok()
            }
        })
        .collect();
    let base_frequency: u64 =
        std::fs::read_to_string("/sys/devices/system/cpu/cpu0/acpi_cppc/nominal_freq")
            .ok()?
            .parse()
            .ok()?;
    let virtualization_enabled = std::fs::File::open("/dev/kvm").is_ok();

    Some(CpuInfo {
        name,
        core_count,
        sockets: sockets.len(),
        base_frequency: Frequency::from_mhz(base_frequency),
        caches: vec![],
        virtualization_enabled,
    })
}

fn parse_cpu_list(cpu_list: &str) -> Option<usize> {
    cpu_list.trim().split(',').try_fold(0usize, |total, entry| {
        let entry = entry.trim();

        let (start, end) = match entry.split_once('-') {
            Some((start, end)) => (
                start.trim().parse::<usize>().ok()?,
                end.trim().parse::<usize>().ok()?,
            ),
            None => {
                let cpu = entry.parse::<usize>().ok()?;
                (cpu, cpu)
            }
        };

        let range_size = end.checked_sub(start)?.checked_add(1)?;
        total.checked_add(range_size)
    })
}

#[cfg(test)]
mod tests {
    use super::parse_cpu_list;

    #[test]
    fn parses_linux_cpu_lists() {
        assert_eq!(parse_cpu_list("0"), Some(1));
        assert_eq!(parse_cpu_list("0-3"), Some(4));
        assert_eq!(parse_cpu_list("0-3,8-11"), Some(8));
        assert_eq!(parse_cpu_list("0,2,4-6\n"), Some(5));
    }

    #[test]
    fn rejects_invalid_cpu_lists() {
        assert_eq!(parse_cpu_list(""), None);
        assert_eq!(parse_cpu_list("3-1"), None);
        assert_eq!(parse_cpu_list("0-invalid"), None);
        assert_eq!(parse_cpu_list("0-1-2"), None);
    }
}
