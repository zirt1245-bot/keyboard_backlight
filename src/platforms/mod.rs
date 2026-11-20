pub mod rainbow;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub use windows as backend;

#[cfg(target_os = "linux")]
pub use linux as backend;

pub trait KeyboardBackend {
    fn set_color(&self, r: u8, g: u8, b: u8) -> Result<(), String>;
    fn set_power(&self, on: bool) -> Result<(), String>;
    fn set_brightness(&self, level: &str) -> Result<(), String>;
    fn set_background(&self, on: bool) -> Result<(), String>;
    fn set_background_breath(&self, color_name: &str) -> Result<(), String>;
    fn set_stop(&self) -> Result<(), String>;
    fn random_color(&self) -> (u8, u8, u8);
}
