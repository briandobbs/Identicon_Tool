use super::colors_data::get_color;
use super::background_colors_data::get_background_color;

#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Debug)]
pub struct Colors {
    pub color: Rgb,
    pub background_color: Rgb
}

pub fn get_colors(input: &u8) -> Colors {

    let result = Colors {
        color: get_color(input),
        background_color: get_background_color(input)
    };

    return result;

}