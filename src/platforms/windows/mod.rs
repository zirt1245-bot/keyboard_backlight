pub mod keyboard;
pub mod wmi;

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

    pub fn set_backgound(&self, on: bool) -> Result<(), String> {
        todo!()
    }

    pub fn set_stop(&self, on: bool) -> Result<(), String> {
        todo!()
    }
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
