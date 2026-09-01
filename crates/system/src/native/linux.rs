pub struct LinuxBackend;

impl super::Backend for LinuxBackend {
    fn new() -> Self {
        Self
    }

    fn core_count(&self) -> Option<usize> {
        None
    }
}
