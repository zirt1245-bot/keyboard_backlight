use nix::unistd::{fork, ForkResult};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use crate::platforms::rainbow::hsv_to_rgb;

pub const LED: &str =
    "/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/multi_intensity";
pub const OFF_ON: &str = "/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/brightness";
pub const PID_FILE: &str = "/tmp/backlight_background.pid";

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

        read_brightness(OFF_ON).parse::<u16>().unwrap_or(128);

        let new_brightness = match level {
            s if s.ends_with('%') => {
                // Процент: 50% → 127
                let percent = s
                    .trim_end_matches('%')
                    .parse::<f32>()
                    .map_err(|_| "Invalid percentage")?;
                ((percent / 100.0) * 255.0) as u16
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

    pub fn set_background(&self, on: bool) -> Result<(), String> {
        if !on {
            return self.stop_daemon();
        }

        if Path::new(PID_FILE).exists() {
            let old_pid = fs::read_to_string(PID_FILE)
                .ok()
                .and_then(|p| p.trim().parse::<u32>().ok());

            if let Some(pid) = old_pid {
                let _ = Command::new("kill").arg("-0").arg(pid.to_string()).output();
            }
        }

        match unsafe { fork() }.map_err(|_| "Fork failed".to_string())? {
            ForkResult::Parent { child } => {
                fs::write(PID_FILE, child.to_string())
                    .map_err(|e| format!("Failed to write PID: {}", e))?;
                Ok(())
            }
            ForkResult::Child => {
                let mut hue = 0.0;
                loop {
                    let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);
                    let color_str = format!("{} {} {}", r, g, b);

                    hue += 0.002;
                    if hue > 1.0 {
                        hue = 0.0;
                    }

                    if let Err(_) = fs::write(LED, &color_str) {
                        break;
                    }

                    sleep(Duration::from_millis(16));
                }
                std::process::exit(0);
            }
        }
    }

    pub fn set_stop(&self) -> Result<(), String> {
        self.stop_daemon()
    }

    fn stop_daemon(&self) -> Result<(), String> {
        if !Path::new(PID_FILE).exists() {
            return Err("No daemon running".to_string());
        }

        let pid = fs::read_to_string(PID_FILE)
            .and_then(|p| {
                p.trim()
                    .parse::<u32>()
                    .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Parse error"))
            })
            .map_err(|e| format!("Failed to read PID: {}", e))?;

        Command::new("kill")
            .arg(pid.to_string())
            .output()
            .map_err(|e| format!("Failed to kill daemon: {}", e))?;

        fs::remove_file(PID_FILE).ok();
        println!("✓ Rainbow daemon stopped");
        Ok(())
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
    } else {
        input
            .parse::<u8>()
            .map_err(|_| "Invalid brightness value".to_string())
    }
}

fn read_brightness(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| "128".to_string())
}
