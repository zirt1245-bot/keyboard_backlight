pub mod raindow;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub use windows as backend;

#[cfg(target_os = "linux")]
pub use linux as backend;

pub trait _KeyboardBackend {
    fn set_color(&self, r: u8, g: u8, b: u8) -> Result<(), String>;
    fn set_power(&self, on: bool) -> Result<(), String>;
    fn set_brightness(&self, level: &str) -> Result<(), String>;
    fn set_backgound(&self, on: bool) -> Result<(), String>;
    fn set_stop(&self) -> Result<(), String>;
}
