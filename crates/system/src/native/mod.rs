#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

pub trait Backend {
    fn new() -> Self;
    fn core_count(&self) -> Option<usize>;
}
