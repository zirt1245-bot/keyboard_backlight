pub mod keyboard;
pub mod wmi;

use std::fs;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::platforms::rainbow::hsv_to_rgb;
use crate::platforms::KeyboardBackend;

const CREATE_NO_WINDOW: u32 = 0x08000000;
const DETACHED_PROCESS: u32 = 0x00000008;

fn get_pid_file() -> String {
    std::env::temp_dir()
        .join("backlight_background.pid")
        .to_string_lossy()
        .to_string()
}

pub struct KeyboardController {
    controller: keyboard::KeyboardController,
}

impl KeyboardBackend for KeyboardController {
    fn set_color(&self, r: u8, g: u8, b: u8) -> Result<(), String> {
        self.controller
            .set_color(keyboard::Rgb::R, r, g, b)
            .and_then(|_| self.controller.set_color(keyboard::Rgb::G, r, g, b))
            .and_then(|_| self.controller.set_color(keyboard::Rgb::B, r, g, b))
            .map_err(|e| format!("{:?}", e))
    }

    fn set_power(&self, on: bool) -> Result<(), String> {
        self.controller
            .set_power(on)
            .map_err(|e| format!("{:?}", e))
    }

    fn set_brightness(&self, level: &str) -> Result<(), String> {
        if let Ok(num) = level.parse::<u8>() {
            if num >= 1 && num <= 4 {
                self.controller
                    .set_brightness(num)
                    .map_err(|e| format!("{:?}", e))
            } else {
                Err("Brightness on Windows: 1 (min) to 4 (max)".to_string())
            }
        } else {
            Err("Invalid brightness value".to_string())
        }
    }

    fn set_background(&self, on: bool) -> Result<(), String> {
        if !on {
            return self.stop_service();
        }
        self.spawn_daemon(&[])
    }

    fn set_background_breath(&self, color_name: &str) -> Result<(), String> {
        self.spawn_daemon(&["--back-breath".to_string(), color_name.to_string()])
    }

    fn set_stop(&self) -> Result<(), String> {
        self.stop_service()
    }
}

impl KeyboardController {
    fn spawn_daemon(&self, extra_args: &[String]) -> Result<(), String> {
        let pid_file = get_pid_file();

        if Path::new(&pid_file).exists() {
            if let Ok(pid_str) = fs::read_to_string(&pid_file) {
                if let Ok(pid) = pid_str.trim().parse::<u32>() {
                    if is_process_running(pid) {
                        return Err("Daemon already running".to_string());
                    }
                }
            }
        }

        let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;

        let child = Command::new(current_exe)
            .arg("--internal-daemon")
            .args(extra_args)
            .creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed to spawn: {}", e))?;

        let pid = child.id();
        fs::write(&pid_file, pid.to_string()).map_err(|e| format!("Failed to write PID: {}", e))?;

        println!("✓ Daemon started (PID: {})", pid);
        Ok(())
    }

    pub fn run_breath(&self, base_r: u8, base_g: u8, base_b: u8) {
        let start_time = Instant::now();

        loop {
            let elapsed = start_time.elapsed().as_secs_f32();

            let phase = (f32::sin(elapsed * 2.0) + 1.0) / 2.0;

            let r = (base_r as f32 * phase).round() as u8;
            let g = (base_g as f32 * phase).round() as u8;
            let b = (base_b as f32 * phase).round() as u8;

            if let Err(e) = self.set_color(r, g, b) {
                eprintln!("✗ Error in the 'breath' cycle: {}", e);
                return;
            }

            sleep(Duration::from_millis(10));
        }
    }

    pub fn run_rainbow(&self) {
        let mut hue = 0.0;
        loop {
            let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);

            if self.set_color(r, g, b).is_err() {
                break;
            }

            hue += 0.002;
            if hue > 1.0 {
                hue = 0.0;
            }

            thread::sleep(Duration::from_millis(16));
        }
    }

    fn stop_service(&self) -> Result<(), String> {
        let pid_file = get_pid_file();

        if !Path::new(&pid_file).exists() {
            return Err("No daemon running".to_string());
        }

        let pid_str =
            fs::read_to_string(&pid_file).map_err(|e| format!("Failed to read PID: {}", e))?;

        let pid = pid_str
            .trim()
            .parse::<u32>()
            .map_err(|_| "Invalid PID file".to_string())?;

        let status = Command::new("taskkill")
            .args(&["/F", "/PID", &pid.to_string()])
            .creation_flags(CREATE_NO_WINDOW)
            .status()
            .map_err(|e| format!("Failed to kill process: {}", e))?;

        fs::remove_file(&pid_file).ok();

        if status.success() {
            println!("✓ Rainbow daemon stopped");
            Ok(())
        } else {
            Err("Failed to stop daemon".to_string())
        }
    }
}

fn is_process_running(pid: u32) -> bool {
    Command::new("tasklist")
        .args(&["/FI", &format!("PID eq {}", pid)])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).contains(&pid.to_string()))
        .unwrap_or(false)
}

pub fn init_backend() -> Result<KeyboardController, String> {
    println!("Initializing WMI connection...");
    match keyboard::KeyboardController::new() {
        Ok(kb) => {
            println!("Connected to CLEVO_GET WMI endpoint");
            Ok(KeyboardController { controller: kb })
        }
        Err(e) => {
            eprintln!("Failed to connect to WMI: {:?}", e);
            eprintln!("Make sure you are running as Administrator");
            Err(format!("WMI initialization failed: {:?}", e))
        }
    }
}
