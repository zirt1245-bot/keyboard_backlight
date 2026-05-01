pub mod keyboard;
pub mod wmi;

use std::fs;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};

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

impl KeyboardController {
    pub fn set_color(&self, r: u8, g: u8, b: u8) -> Result<(), String> {
        self.controller
            .set_color(keyboard::Rgb::R, r, g, b)
            .and_then(|_| self.controller.set_color(keyboard::Rgb::G, r, g, b))
            .and_then(|_| self.controller.set_color(keyboard::Rgb::B, r, g, b))
            .map_err(|e| format!("{:?}", e))
    }

    pub fn set_power(&self, on: bool) -> Result<(), String> {
        self.controller
            .set_power(on)
            .map_err(|e| format!("{:?}", e))
    }

    pub fn set_brightness(&self, level: &str) -> Result<(), String> {
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

    pub fn set_background(&self, on: bool) -> Result<(), String> {
        if !on {
            return self.stop_service();
        }

        let pid_file = get_pid_file();

        if Path::new(&pid_file).exists() {
            if let Ok(pid_str) = fs::read_to_string(&pid_file) {
                if let Ok(pid) = pid_str.trim().parse::<u32>() {
                    if is_process_running(pid) {
                        return Err("Rainbow daemon already running".to_string());
                    }
                }
            }
        }

        let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;

        let mut child = Command::new(current_exe)
            .arg("--internal-daemon")
            .creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| format!("Failed to spawn daemon: {}", e))?;

        let pid = child.id();
        fs::write(&pid_file, pid.to_string()).map_err(|e| format!("Failed to write PID: {}", e))?;

        println!("✓ Rainbow daemon started (PID: {})", pid);
        Ok(())
    }

    pub fn set_stop(&self) -> Result<(), String> {
        self.stop_service()
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
