use serde_derive::{Deserialize, Serialize};

use super::colors_data::get_colors_data;
use super::background_colors_data::get_background_colors_data;

#[derive(Deserialize, Serialize, Debug)]
pub struct Color {
    pub name: String,
    pub rgb: Rgb
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Debug)]
pub struct Colors {
    pub color: Color,
    pub background_color: Color
}

pub fn get_colors(input: &u8) -> Colors {

    let colors_data = get_colors_data();
    let background_colors_data = get_background_colors_data();

    let color_index = get_color_index(input);
    let background_color_index = get_background_color_index(input);

    let result = Colors {
        color: serde_json::from_str::<Vec<Color>>(&colors_data).unwrap().into_iter().nth(color_index.into()).unwrap(),
        background_color: serde_json::from_str::<Vec<Color>>(&background_colors_data).unwrap().into_iter().nth(background_color_index.into()).unwrap()
    };

    return result;

}

fn get_color_index(input: &u8) -> u8 {
    let result = input % 17;
    return result;
}

fn get_background_color_index(input: &u8) -> u8 {
    if let 0 = input % 2 {
        // result is even
        return 0;
    } else {
        // result is odd
        return 1;
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_colors() {
        let input: u8 = 68;
        let result = get_colors(&input);
        assert_eq!(result.color.name, "red");
        assert_eq!(result.background_color.name, "gray");
    }

    #[test]
    fn test_get_color_index() {
        let input: u8 = 68;
        let color_index = get_color_index(&input);
        println!("{}", color_index);
        assert_eq!(color_index, 15);
    }

    #[test]
    fn test_get_background_color_index_even() {
        let input: u8 = 48;
        let background_color_index = get_background_color_index(&input);
        assert_eq!(background_color_index, 0);
    }

    #[test]
    fn test_get_background_color_index_odd() {
        let input: u8 = 69;
        let background_color_index = get_background_color_index(&input);
        assert_eq!(background_color_index, 1);
    }

}

