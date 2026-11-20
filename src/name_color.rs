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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_colors() {
        assert_eq!(get_color_by_name("red"), Ok((255, 0, 0)));
        assert_eq!(get_color_by_name("green"), Ok((0, 255, 0)));
        assert_eq!(get_color_by_name("blue"), Ok((0, 0, 255)));
        assert_eq!(get_color_by_name("white"), Ok((255, 255, 255)));
        assert_eq!(get_color_by_name("yellow"), Ok((255, 255, 0)));
        assert_eq!(get_color_by_name("cyan"), Ok((0, 255, 255)));
        assert_eq!(get_color_by_name("magenta"), Ok((255, 0, 255)));
        assert_eq!(get_color_by_name("orange"), Ok((255, 128, 0)));
        assert_eq!(get_color_by_name("pink"), Ok((255, 0, 127)));
        assert_eq!(get_color_by_name("turquoise"), Ok((0, 128, 128)));
        assert_eq!(get_color_by_name("violet"), Ok((127, 0, 255)));
        assert_eq!(get_color_by_name("lime"), Ok((127, 255, 0)));
        assert_eq!(get_color_by_name("golden"), Ok((255, 180, 50)));
        assert_eq!(get_color_by_name("purple"), Ok((128, 0, 255)));
        assert_eq!(get_color_by_name("brown"), Ok((165, 42, 42)));
        assert_eq!(get_color_by_name("light_blue"), Ok((173, 216, 230)));
    }

    #[test]
    fn test_gray_and_grey_aliases() {
        assert_eq!(get_color_by_name("gray"), Ok((100, 100, 100)));
        assert_eq!(get_color_by_name("grey"), Ok((100, 100, 100)));
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(get_color_by_name("RED"), Ok((255, 0, 0)));
        assert_eq!(get_color_by_name("Green"), Ok((0, 255, 0)));
        assert_eq!(get_color_by_name("BLUE"), Ok((0, 0, 255)));
        assert_eq!(get_color_by_name("White"), Ok((255, 255, 255)));
    }

    #[test]
    fn test_unknown_color_returns_error() {
        let result = get_color_by_name("nonexistent");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("nonexistent"));
    }

    #[test]
    fn test_empty_string_returns_error() {
        let result = get_color_by_name("");
        assert!(result.is_err());
    }
}
