use nix::unistd::{fork, setsid, ForkResult};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use crate::platforms::rainbow::hsv_to_rgb;
use crate::platforms::KeyboardBackend;

pub const LED: &str =
    "/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/multi_intensity";
pub const OFF_ON: &str = "/sys/devices/platform/tuxedo_keyboard/leds/rgb:kbd_backlight/brightness";
pub const PID_FILE: &str = "/tmp/backlight_background.pid";

pub struct KeyboardController;

impl KeyboardBackend for KeyboardController {
    fn set_color(&self, r: u8, g: u8, b: u8) -> Result<(), String> {
        let color_str = format!("{} {} {}", r, g, b);
        fs::write(LED, &color_str).map_err(|e| format!("Failed to write color: {}", e))
    }

    fn set_power(&self, on: bool) -> Result<(), String> {
        let value = if on { "255" } else { "0" };
        fs::write(OFF_ON, value).map_err(|e| format!("Failed to set power: {}", e))
    }

    /// Установка яркости. На Tuxedo клавиатуре яркость и включение/выключение
    /// пишут в один и тот же sysfs-файл (`brightness`), поэтому установка
    /// ненулевой яркости автоматически включает подсветку.
    fn set_brightness(&self, level: &str) -> Result<(), String> {
        let new_brightness = match level {
            s if s.ends_with('%') => {
                // Процент: 50% → ~127
                let percent = s
                    .trim_end_matches('%')
                    .parse::<f32>()
                    .map_err(|_| "Invalid percentage")?;
                ((percent / 100.0) * 255.0) as u16
            }
            s => {
                // Абсолютное значение 0..=255
                s.parse::<u16>().map_err(|_| "Invalid brightness value")?
            }
        };

        let final_brightness = (new_brightness.min(255)) as u8;
        fs::write(OFF_ON, final_brightness.to_string())
            .map_err(|e| format!("Failed to set brightness: {}", e))
    }

    fn set_background(&self, on: bool) -> Result<(), String> {
        if !on {
            return self.stop_daemon();
        }

        // Проверяем, не запущен ли уже демон
        if Path::new(PID_FILE).exists() {
            if let Ok(pid_str) = fs::read_to_string(PID_FILE) {
                if let Ok(pid) = pid_str.trim().parse::<u32>() {
                    // kill -0 не посылает сигнал, только проверяет существование процесса
                    if let Ok(output) =
                        Command::new("kill").arg("-0").arg(pid.to_string()).output()
                    {
                        if output.status.success() {
                            return Err("Background daemon already running".to_string());
                        }
                    }
                }
            }
        }

        match unsafe { fork() }.map_err(|_| "Fork failed".to_string())? {
            ForkResult::Parent { child } => {
                // Создаём PID-файл эксклюзивно (защита от гонки)
                let mut pid_file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(PID_FILE)
                    .map_err(|e| {
                        format!(
                            "Failed to create PID file (daemon may already be running): {}",
                            e
                        )
                    })?;
                writeln!(pid_file, "{}", child)
                    .map_err(|e| format!("Failed to write PID: {}", e))?;
                Ok(())
            }
            ForkResult::Child => {
                // Отсоединяемся от управляющего терминала, чтобы демон не умирал
                // при закрытии сессии и не hielt stdin/stdout родителя
                let _ = setsid();

                let mut hue = 0.0;
                loop {
                    let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);
                    let color_str = format!("{} {} {}", r, g, b);

                    hue += 0.002;
                    if hue >= 1.0 {
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

    fn set_stop(&self) -> Result<(), String> {
        self.stop_daemon()
    }
}

impl KeyboardController {
    fn stop_daemon(&self) -> Result<(), String> {
        if !Path::new(PID_FILE).exists() {
            return Err("No daemon running".to_string());
        }

        let pid_str = fs::read_to_string(PID_FILE)
            .map_err(|e| format!("Failed to read PID file: {}", e))?;
        let pid = pid_str
            .trim()
            .parse::<u32>()
            .map_err(|_| "Invalid PID file".to_string())?;

        // Проверяем, жив ли процесс, перед отправкой сигнала
        let check = Command::new("kill").arg("-0").arg(pid.to_string()).output();
        if let Ok(output) = check {
            if !output.status.success() {
                // Процесс уже мёртв — просто чистим PID-файл
                let _ = fs::remove_file(PID_FILE);
                return Err("No daemon running".to_string());
            }
        }

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
        return Err(
            "❌ Backlight control file not found.\n\
             Install Tuxedo Linux Driver or check if /sys paths are accessible"
                .to_string(),
        );
    }

    println!("Tuxedo Driver detected");
    Ok(KeyboardController)
}
