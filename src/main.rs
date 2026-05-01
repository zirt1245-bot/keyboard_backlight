mod name_color;
mod platforms;

use clap::Parser;
use name_color::get_color_by_name;
use platforms::backend;

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

    /// Backgound raindow (linux only)
    #[arg(long)]
    back_rainbow: bool,

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

    if cli.off {
        match kb.set_power(false) {
            Ok(ok) => ok,
            Err(e) => eprintln!("✗ Error turning off: {}", e),
        }
    } else if cli.on {
        match kb.set_power(true) {
            Ok(ok) => ok,
            Err(e) => eprintln!("✗ Error turning on: {}", e),
        }
    }

    if let Some(rgb_values) = cli.rgb {
        if rgb_values.len() == 3 {
            match kb.set_color(rgb_values[0], rgb_values[1], rgb_values[2]) {
                Ok(ok) => ok,
                Err(e) => eprintln!("✗ Error setting color: {}", e),
            }
        }
    }

    if let Some(color_name) = cli.color {
        let (r, g, b) = get_color_by_name(&color_name);
        match kb.set_color(r, g, b) {
            Ok(ok) => ok,
            Err(e) => eprintln!("✗ Error: {}", e),
        }
    }

    if let Some(brightness_input) = cli.brightness {
        match kb.set_brightness(&brightness_input) {
            Ok(ok) => ok,
            Err(e) => eprintln!("✗ Error: {}", e),
        }
    }

    if cli.back_rainbow {
        match kb.set_background(true) {
            Ok(ok) => ok,
            Err(e) => eprintln!("✗ Error setting color: {}", e),
        }
    }

    if cli.stop {
        match kb.set_stop() {
            Ok(ok) => ok,
            Err(e) => eprintln!("✗ Error stopping rainbow: {}", e),
        }
    }

    #[cfg(target_os = "windows")]
    if cli.internal_daemon {
        kb.run_service();
        return;
    }

    println!("Done!");
}
