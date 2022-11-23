use serde_derive::{Deserialize, Serialize};
use std::fs;

// #[derive(Debug)]
// pub struct ColorError {
//     details: String
// }

// impl ColorError {
//     fn new(msg: &str) -> ColorError {
//         ColorError{details: msg.to_string()}
//     }
// }

// impl fmt::Display for ColorError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f,"{}",self.details)
//     }
// }

// impl Error for ColorError {
//     fn description(&self) -> &str {
//         &self.details
//     }
// }

#[derive(Deserialize, Serialize, Debug)]
pub struct Color {
    pub name: String,
    pub rgb: Rgb,
    pub hex: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub struct Colors {
    pub color: Color,
    pub background_color: Color
}

pub fn get_colors(input: &str) -> Colors {

    let colors_data = fs::read_to_string("./src/identicon/colors.json").unwrap();
    let background_colors_data = fs::read_to_string("./src/identicon/background_colors.json").unwrap();

    let color_index = get_color_index(input);
    let background_color_index = get_background_color_index(input);

    let result = Colors {
        color: serde_json::from_str::<Vec<Color>>(&colors_data).unwrap().into_iter().nth(color_index.into()).unwrap(),
        background_color: serde_json::from_str::<Vec<Color>>(&background_colors_data).unwrap().into_iter().nth(background_color_index.into()).unwrap()
    };

    return result;

}

fn get_color_index(input: &str) -> u8 {
    let first_char_u8_value: u8 = input.chars().nth(0).unwrap() as u8;
    let result = first_char_u8_value % 17;
    return result;
}

fn get_background_color_index(input: &str) -> u8 {
    let first_char_u8_value: u8 = input.chars().nth(0).unwrap() as u8;
    let result: u8 = if let 0 = first_char_u8_value % 2 {
        // result is even
        return 0;
    } else {
        // result is odd
        return 1;
    };
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_colors() {
        let result = get_colors("D");
        assert_eq!(result.color.name, "red");
        assert_eq!(result.background_color.name, "gray");
    }

    #[test]
    fn test_get_color_index() {
        let color_index = get_color_index("Bello all!");
        println!("{}", color_index);
        assert_eq!(color_index, 15);
    }

    #[test]
    fn test_get_background_color_index_even() {
        let background_color_index = get_background_color_index("0");
        assert_eq!(background_color_index, 0);
    }

    #[test]
    fn test_get_background_color_index_odd() {
        let background_color_index = get_background_color_index("E");
        assert_eq!(background_color_index, 1);
    }

}

