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
        self.parse_cpu_list(&cpu_list)
    }

    fn handle_count(&self) -> Option<usize> {
        std::fs::read_to_string("/proc/sys/fs/file-nr")
            .ok()?
            .split_whitespace()
            .next()?
            .parse()
            .ok()
    }

    fn cpu_info(&self) -> CpuInfo {
        let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") else {
            return CpuInfo::default();
        };
        self.parse_cpuinfo(&cpuinfo)
    }
}

impl LinuxBackend {
    fn parse_cpuinfo(&self, cpuinfo: &str) -> CpuInfo {
        let name = self.parse_cpu_name(cpuinfo);
        let core_count = System::physical_core_count();
        let socket_count = self.parse_socket_count(cpuinfo);
        let base_frequency = self.base_frequency();
        let virtualization_enabled = self.virtualization_enabled();

        CpuInfo {
            name,
            core_count,
            socket_count,
            base_frequency,
            caches: None,
            virtualization_enabled,
        }
    }

    fn parse_cpu_name(&self, cpuinfo: &str) -> Option<String> {
        cpuinfo
            .lines()
            .find(|l| l.starts_with("model name"))?
            .split_once(':')
            .map(|(_, name)| name.trim().to_string())
    }

    fn parse_socket_count(&self, cpuinfo: &str) -> Option<usize> {
        Some(
            cpuinfo
                .lines()
                .filter_map(|l| {
                    if !l.starts_with("physical id") {
                        None
                    } else {
                        l.split_once(':')?.1.trim().parse().ok()
                    }
                })
                .collect::<HashSet<usize>>()
                .len(),
        )
    }

    fn base_frequency(&self) -> Option<Frequency> {
        let base_frequency: u64 =
            std::fs::read_to_string("/sys/devices/system/cpu/cpu0/acpi_cppc/nominal_freq")
                .ok()?
                .trim()
                .parse()
                .ok()?;
        Some(Frequency::from_mhz(base_frequency))
    }

    fn virtualization_enabled(&self) -> Option<bool> {
        std::fs::File::open("/dev/kvm").map(|_| true).ok()
    }

    fn parse_cpu_list(&self, cpu_list: &str) -> Option<usize> {
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
