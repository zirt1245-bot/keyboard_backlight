/// High-level keyboard backlight controller.
///
/// All WMI command codes are reverse-engineered from LEDLib.dll
/// (LED Manager for Clevo by rzrwolf).
use crate::platforms::windows::wmi::WmiConnection;
use windows::core::Result;

const R_RGB: u64 = 0xF000_0000; // -268435456 as i32
const G_RGB: u64 = 0xF200_0000; // -234881024 as i32
const B_RGB: u64 = 0xF100_0000; // -251658240 as i32

// ── KB power ─────────────────────────────────────────────────────────────────
const KB_DISABLE: u32 = 3_758_096_391; // wmiDisableKBcode
const KB_ENABLE: u32 = 3_758_616_577; // wmiEnableKBcode

// ── Brightness levels (index 1..=4) ─────────────────────────────────────────
// Index 0 = "no code, disable kb" (handled via KB_DISABLE)
const BRIGHTNESS: [u32; 5] = [
    0,             // 0: unused (disable via KB_DISABLE instead)
    4_093_640_751, // 1: minimum brightness
    4_093_640_799, // 2
    4_093_640_847, // 3
    4_093_640_895, // 4: maximum brightness
];

#[derive(Debug, Clone, Copy)]
pub enum Rgb {
    R,
    G,
    B,
}

pub struct KeyboardController {
    wmi: WmiConnection,
}

impl KeyboardController {
    pub fn new() -> Result<Self> {
        Ok(Self {
            wmi: WmiConnection::new()?,
        })
    }

    pub fn set_color(&self, rgb: Rgb, r: u8, g: u8, b: u8) -> Result<()> {
        let zone_flag = match rgb {
            Rgb::R => R_RGB,
            Rgb::G => G_RGB,
            Rgb::B => B_RGB,
        };

        // Scale blue to 0-200 range (as LEDLib does)
        let b_scaled = (b as f64 * 200.0 / 255.0).round() as u64;

        let packet: u64 = (b_scaled << 16) | (g as u64) | ((r as u64) << 8);
        let packet = (packet | zone_flag) as u32;

        self.wmi.invoke_kb(packet)
    }

    /// Set brightness level 1 (min) to 4 (max).
    pub fn set_brightness(&self, level: u8) -> Result<()> {
        let level = level.clamp(1, 4) as usize;
        self.wmi.invoke_kb(BRIGHTNESS[level])
    }

    /// Turn keyboard LED on or off.
    pub fn set_power(&self, on: bool) -> Result<()> {
        let code = if on { KB_ENABLE } else { KB_DISABLE };
        self.wmi.invoke_kb(code)
    }
}
