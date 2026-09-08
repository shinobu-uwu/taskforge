use crate::cpu::CpuInfo;
use crate::native::Backend;
use sysinfo::System;
use windows::Win32::System::ProcessStatus::{GetPerformanceInfo, PERFORMANCE_INFORMATION};
use windows::Win32::System::Threading::{ALL_PROCESSOR_GROUPS, GetActiveProcessorCount};
use windows::Win32::System::WindowsProgramming::GetComputerNameA;

#[derive(Debug, Clone)]
pub struct WindowsBackend;

impl Backend for WindowsBackend {
    fn new() -> Self {
        Self
    }

    fn core_count(&self) -> Option<usize> {
        System::physical_core_count()
    }

    fn logical_processor_count(&self) -> Option<usize> {
        unsafe { Some(GetActiveProcessorCount(ALL_PROCESSOR_GROUPS) as usize) }
    }

    fn handle_count(&self) -> Option<usize> {
        unsafe {
            let mut info = PERFORMANCE_INFORMATION::default();

            GetPerformanceInfo(&mut info, size_of::<PERFORMANCE_INFORMATION>() as u32).ok()?;

            Some(info.HandleCount as usize)
        }
    }

    fn cpu_info(&self) -> CpuInfo {
        todo!()
    }

    fn thread_count(&self) -> Option<usize> {
        unsafe {
            let mut info = PERFORMANCE_INFORMATION::default();

            GetPerformanceInfo(&mut info, size_of::<PERFORMANCE_INFORMATION>() as u32).ok()?;

            Some(info.ThreadCount as usize)
        }
    }
}
