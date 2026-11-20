mod name_color;
mod platforms;

use clap::Parser;
use name_color::get_color_by_name;
use platforms::backend;
use platforms::KeyboardBackend;

#[derive(Parser, Debug)]
#[command(
    name = "Backlight",
    version,
    about = "Changes the color of the keyboard"
)]
struct Cli {
    /// Your RGB: --rgb 0 0 0
    #[arg(long, num_args = 3, value_names = ["RED", "GREEN", "BLUE"])]
    rgb: Option<Vec<u8>>,

    /// Turn backlight off
    #[arg(long)]
    off: bool,

    /// Turn backlight on
    #[arg(long)]
    on: bool,

    /// Color by name: red, green, blue, etc
    #[arg(short, long)]
    color: Option<String>,

    /// Brightness (Windows: 1-4, Linux: 0-255 or 0%-100%)
    #[arg(short, long)]
    brightness: Option<String>,

    /// Background rainbow
    #[arg(long)]
    back_rainbow: bool,

    /// Background breathing with your color: red, green, blue, etc (white by default, Windows only)
    #[arg(long, num_args = 0..=1, default_missing_value = "white")]
    back_breath: Option<String>,

    /// Stop backgound effect
    #[arg(long)]
    stop: bool,

    #[arg(long, hide = true)]
    internal_daemon: bool,
}

fn main() {
    let cli = Cli::parse();

    let kb = match backend::init_backend() {
        Ok(controller) => controller,
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            std::process::exit(1);
        }
    };

    #[cfg(target_os = "windows")]
    if cli.internal_daemon {
        match cli.back_breath {
            Some(color_name) => match get_color_by_name(&color_name) {
                Ok((r, g, b)) => kb.run_breath(r, g, b),
                Err(e) => {
                    eprintln!("✗ {}", e);
                    std::process::exit(1);
                }
            },
            None => kb.run_rainbow(),
        }
        return;
    }

    #[cfg(target_os = "linux")]
    if cli.back_breath.is_some() {
        eprintln!("✗ Error: --back-breath is only supported on Windows");
        std::process::exit(1);
    }

    if cli.off {
        if let Err(e) = kb.set_power(false) {
            eprintln!("✗ Error turning off: {}", e);
        }
    } else if cli.on {
        if let Err(e) = kb.set_power(true) {
            eprintln!("✗ Error turning on: {}", e);
        }
    }

    if let Some(rgb_values) = cli.rgb {
        if rgb_values.len() == 3 {
            if let Err(e) = kb.set_color(rgb_values[0], rgb_values[1], rgb_values[2]) {
                eprintln!("✗ Error setting color: {}", e);
            }
        }
    }

    if let Some(color_name) = cli.color {
        match get_color_by_name(&color_name) {
            Ok((r, g, b)) => {
                if let Err(e) = kb.set_color(r, g, b) {
                    eprintln!("✗ Error setting color: {}", e);
                }
            }
            Err(e) => eprintln!("✗ {}", e),
        }
    }

    if let Some(brightness_input) = cli.brightness {
        if let Err(e) = kb.set_brightness(&brightness_input) {
            eprintln!("✗ Error: {}", e);
        }
    }

    if cli.back_rainbow {
        if let Err(e) = kb.set_background(true) {
            eprintln!("✗ Error setting background: {}", e);
        }
    }

    #[cfg(target_os = "windows")]
    if let Some(color_name) = cli.back_breath {
        match get_color_by_name(&color_name) {
            Ok(_) => {
                if let Err(e) = kb.set_background_breath(&color_name) {
                    eprintln!("✗ Error setting background breath: {}", e);
                }
            }
            Err(e) => eprintln!("✗ {}", e),
        }
    }

    if cli.stop {
        if let Err(e) = kb.set_stop() {
            eprintln!("✗ Error stopping background: {}", e);
        }
    }

    println!("Done!");
}
