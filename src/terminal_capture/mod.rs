pub mod traits;

#[cfg(target_family = "unix")]
pub mod unix;

#[cfg(target_family = "windows")]
pub mod windows;

#[cfg(target_family = "unix")]
pub use unix::UnixTerminalCapture;

#[cfg(target_family = "windows")]
pub use windows::WindowsTerminalCapture;

pub fn capture_instance() -> Box<dyn TerminalCapture> {
    if cfg!(target_family = "unix") {
        Box::new(UnixTerminalCapture)
    } else if cfg!(target_family = "windows") {
        Box::new(WindowsTerminalCapture)
    } else {
        panic!("Unsupported platform");
    }
}