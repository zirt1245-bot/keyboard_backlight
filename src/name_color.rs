use colored::Colorize;

pub fn get_color_by_name(name: &str) -> Result<(u8, u8, u8), String> {
    match name.to_lowercase().as_str() {
        "red" => Ok((255, 0, 0)),
        "green" => Ok((0, 255, 0)),
        "blue" => Ok((0, 0, 255)),
        "white" => Ok((255, 255, 255)),
        "yellow" => Ok((255, 255, 0)),
        "cyan" => Ok((0, 255, 255)),
        "magenta" => Ok((255, 0, 255)),
        "orange" => Ok((255, 128, 0)),
        "pink" => Ok((255, 0, 127)),
        "turquoise" => Ok((0, 128, 128)),
        "violet" => Ok((127, 0, 255)),
        "lime" => Ok((127, 255, 0)),
        "golden" => Ok((255, 180, 50)),
        "gray" | "grey" => Ok((100, 100, 100)),
        "purple" => Ok((128, 0, 255)),
        "brown" => Ok((165, 42, 42)),
        "light_blue" => Ok((173, 216, 230)),
        _ => {
            show_available_colors(name);
            Err(format!("Unknown color: '{}'", name))
        }
    }
}

fn show_available_colors(_unknown: &str) {
    println!(
        "\n Available colors:\n  {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}\n",
        "red".red(),
        "green".green(),
        "blue".blue(),
        "white".truecolor(255, 255, 255),
        "yellow".yellow(),
        "cyan".cyan(),
        "magenta".magenta(),
        "orange".truecolor(255, 128, 0),
        "pink".truecolor(255, 0, 127),
        "turquoise".truecolor(0, 128, 128),
        "violet".truecolor(127, 0, 255),
        "lime".truecolor(127, 255, 0),
        "golden".truecolor(255, 180, 50),
        "purple".truecolor(128, 0, 255),
        "brown".truecolor(165, 42, 42),
        "light_blue".truecolor(173, 216, 230),
        "gray".normal()
    );
}
