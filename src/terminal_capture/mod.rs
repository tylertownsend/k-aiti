pub mod traits;

#[cfg(target_family = "unix")]
pub mod unix;

#[cfg(target_family = "windows")]
pub mod windows;

#[cfg(target_family = "unix")]
pub use unix::UnixTerminalCapture as TerminalCaptureInstance;

#[cfg(target_family = "windows")]
pub use windows::WindowsTerminalCapture as TerminalCaptureInstance;

pub fn capture_instance() -> Box<dyn traits::TerminalCapture> {
    Box::new(TerminalCaptureInstance)
}