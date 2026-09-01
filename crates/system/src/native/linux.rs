use sysinfo::System;

#[derive(Debug, Clone)]
pub struct LinuxBackend;

impl super::Backend for LinuxBackend {
    fn new() -> Self {
        Self
    }

    fn core_count(&self) -> Option<usize> {
        System::physical_core_count()
    }

    fn thread_count(&self) -> Option<usize> {
        None
    }

    fn descriptor_count(&self) -> Option<usize> {
        None
    }
}
