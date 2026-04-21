use std::fs;
use std::path::Path;

pub const LED: &str =
    "/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/multi_intensity";
pub const OFF_ON: &str = "/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/brightness";

pub struct KeyboardController;

impl KeyboardController {
    pub fn set_color(&self, r: u8, g: u8, b: u8) -> Result<(), String> {
        let color_str = format!("{} {} {}", r, g, b);
        fs::write(LED, &color_str).map_err(|e| format!("Failed to write color: {}", e))
    }

    pub fn set_power(&self, on: bool) -> Result<(), String> {
        let value = if on { "255" } else { "0" };
        fs::write(OFF_ON, value).map_err(|e| format!("Failed to set power: {}", e))
    }

    pub fn set_brightness(&self, level: &str) -> Result<(), String> {
        parse_brightness_linux(level)?;

        let current = read_brightness(OFF_ON).parse::<u16>().unwrap_or(128);

        let new_brightness = match level {
            s if s.ends_with('%') => {
                // Процент: 50% → 127
                let percent = s
                    .trim_end_matches('%')
                    .parse::<f32>()
                    .map_err(|_| "Invalid percentage")?;
                ((percent / 100.0) * 255.0) as u16
            }
            s if s.starts_with('+') || s.starts_with('-') => {
                // Относительное: +10, -20
                let delta: i16 = s.parse().map_err(|_| "Invalid relative value")?;
                ((current as i16 + delta).max(0).min(255)) as u16
            }
            s => {
                // Абсолютное: 128
                s.parse::<u16>().map_err(|_| "Invalid brightness value")?
            }
        };

        let final_brightness = (new_brightness.min(255)) as u8;
        fs::write(OFF_ON, final_brightness.to_string())
            .map_err(|e| format!("Failed to set brightness: {}", e))
    }
}

pub fn init_backend() -> Result<KeyboardController, String> {
    println!("Checking Tuxedo Driver...");

    if !Path::new(LED).exists() {
        return Err("❌ Backlight control file not found.\n\
             Install Tuxedo Linux Driver or check if /sys paths are accessible"
            .to_string());
    }

    println!("Tuxedo Driver detected");
    Ok(KeyboardController)
}

fn parse_brightness_linux(input: &str) -> Result<u8, String> {
    if input.ends_with('%') {
        let percent = input
            .trim_end_matches('%')
            .parse::<f32>()
            .map_err(|_| "Invalid percentage")?;
        Ok(((percent / 100.0) * 255.0) as u8)
    } else if input.starts_with('+') || input.starts_with('-') {
        Ok(0)
    } else {
        input
            .parse::<u8>()
            .map_err(|_| "Invalid brightness value".to_string())
    }
}

fn read_brightness(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "128".to_string())
}
