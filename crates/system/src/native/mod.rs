#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
pub use linux::LinuxBackend as NativeBackend;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub use windows::WindowsBackend as NativeBackend;

pub trait Backend {
    fn new() -> Self;
    fn core_count(&self) -> Option<usize>;
    fn thread_count(&self) -> Option<usize>;
    fn descriptor_count(&self) -> Option<usize>;
}
