
use super::algorithm::Identicon;
use super::utils::{is_even, hash_input};
use super::color::get_colors;
use image::{RgbImage, Rgb};

pub struct ColorfulAlgorithm<'a> {
    pub input: &'a str
}

#[derive(Copy, Clone)]
struct RgbValues {
    r: u8,
    g: u8,
    b: u8
}

pub struct FillCoordinates {
    start_corner: CornerCoordinates,
    stop_corner: CornerCoordinates,
    rgb: RgbValues
}

struct CornerCoordinates {
    x: u32,
    y: u32
}

impl Identicon for ColorfulAlgorithm<'_> {

    fn generate(&self) -> RgbImage {

        let hash_string = hash_input(self.input.to_string());

        let pixel_map_input = hash_string.chars().collect();

        let pixel_map = build_pixel_map(pixel_map_input);

        let mut img = RgbImage::new(240, 240);

        for fill in pixel_map.iter() {
            for x in fill.start_corner.x..fill.stop_corner.x {
                for y in fill.start_corner.y..fill.stop_corner.y  {
                    img.put_pixel(x, y, Rgb([fill.rgb.r, fill.rgb.g, fill.rgb.b]));
                }
            }
        }

        img
    }
}

fn build_pixel_map(input: Vec<char>) -> Vec<FillCoordinates> {
    let mut result = Vec::new();

    let intermediate_grid: Vec<&[char]> = input.chunks(8).collect();

    // Create top left quadrant
    for (grid_index, grid_element) in intermediate_grid.iter().enumerate() {
        for (i, x) in grid_element.iter().enumerate() {

            let colors = get_colors(&x.to_string());

            let rgb = RgbValues {
                r: colors.color.rgb.r,
                g: colors.color.rgb.g,
                b: colors.color.rgb.b
            };
        
            let rgb_background = RgbValues {
                r: colors.background_color.rgb.r,
                g: colors.background_color.rgb.g,
                b: colors.background_color.rgb.b
            };

            let start_corner_x: u32 = (i * 15).try_into().unwrap();
            let start_corner_y: u32 = (grid_index * 15).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 15;
            let stop_corner_y: u32 = start_corner_y + 15;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                // if x as u8 is even, we will fill that square with background color
                &x if is_even(x as u8) => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb_background
                    });
                }
                _ => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb
                    });
                }
            } 
        }
    }

    // Create bottom left quadrant
    for (grid_index, grid_element) in intermediate_grid.iter().enumerate() {
        for (i, x) in grid_element.iter().enumerate() {

            let colors = get_colors(&x.to_string());

            let rgb = RgbValues {
                r: colors.color.rgb.r,
                g: colors.color.rgb.g,
                b: colors.color.rgb.b
            };
        
            let rgb_background = RgbValues {
                r: colors.background_color.rgb.r,
                g: colors.background_color.rgb.g,
                b: colors.background_color.rgb.b
            };

            let start_corner_x: u32 = (i * 15).try_into().unwrap();
            let start_corner_y: u32 = (225 - (grid_index * 15)).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 15;
            let stop_corner_y: u32 = start_corner_y + 15;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                // if x as u8 is even, we will fill that square with background color
                &x if is_even(x as u8) => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb_background
                    });
                }
                _ => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb
                    });
                }
            } 
        }
    }
    
    // Create top right quadrant
    for (grid_index, grid_element) in intermediate_grid.iter().enumerate() {
        for (i, x) in grid_element.iter().enumerate() {

            let colors = get_colors(&x.to_string());

            let rgb = RgbValues {
                r: colors.color.rgb.r,
                g: colors.color.rgb.g,
                b: colors.color.rgb.b
            };
        
            let rgb_background = RgbValues {
                r: colors.background_color.rgb.r,
                g: colors.background_color.rgb.g,
                b: colors.background_color.rgb.b
            };

            let start_corner_x: u32 = (225 - (i * 15)).try_into().unwrap();
            let start_corner_y: u32 = (grid_index * 15).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 15;
            let stop_corner_y: u32 = start_corner_y + 15;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                // if x as u8 is even, we will fill that square with background color
                &x if is_even(x as u8) => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb_background
                    });
                }
                _ => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb
                    });
                }
            } 
        }
    }

    // Create bottom right quadrant
    for (grid_index, grid_element) in intermediate_grid.iter().enumerate() {
        for (i, x) in grid_element.iter().enumerate() {

            let colors = get_colors(&x.to_string());

            let rgb = RgbValues {
                r: colors.color.rgb.r,
                g: colors.color.rgb.g,
                b: colors.color.rgb.b
            };
        
            let rgb_background = RgbValues {
                r: colors.background_color.rgb.r,
                g: colors.background_color.rgb.g,
                b: colors.background_color.rgb.b
            };

            let start_corner_x: u32 = (225 - (i * 15)).try_into().unwrap();
            let start_corner_y: u32 = (225 - (grid_index * 15)).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 15;
            let stop_corner_y: u32 = start_corner_y + 15;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                // if x as u8 is even, we will fill that square with background color
                &x if is_even(x as u8) => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb_background
                    });
                }
                _ => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb
                    });
                }
            } 
        }
    }

    return result;
}