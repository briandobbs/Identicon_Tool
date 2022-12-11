use image::{RgbImage, Rgb};

use super::algorithm::Identicon;
use super::color::{get_colors, Colors};
use super::utils::is_even;

pub struct FillCoordinates {
    start_corner: CornerCoordinates,
    stop_corner: CornerCoordinates,
    rgb: RgbValues
}

#[derive(Copy, Clone)]
struct RgbValues {
    r: u8,
    g: u8,
    b: u8
}

struct CornerCoordinates {
    x: u32,
    y: u32
}

pub struct DefaultAlgorithm<'a> {
    pub input: &'a Vec<u8>
}

impl Identicon for DefaultAlgorithm<'_> {
    
    fn generate(&self) -> RgbImage {
        let pixel_map_input = create_pixel_map_input(self.input.to_vec());

        let colors = get_colors(&self.input[0]);

        let pixel_map = build_pixel_map(pixel_map_input, colors);

        let mut img = RgbImage::new(250, 250);

    
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

fn create_pixel_map_input(input: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = input.clone();
    result.extend(input.clone());
    result.extend(input);
    result.truncate(144);

    return result;
}

fn build_pixel_map(input: Vec<u8>, colors: Colors) -> Vec<FillCoordinates> {
    let mut result = Vec::new();

    let intermediate_grid: Vec<&[u8]> = input.chunks(12).collect();

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
        

    // Create top left quadrant
    for (grid_index, grid_element) in intermediate_grid.iter().enumerate() {
        for (i, x) in grid_element.iter().enumerate() {
            let start_corner_x: u32 = (i * 10).try_into().unwrap();
            let start_corner_y: u32 = (grid_index * 10).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 10;
            let stop_corner_y: u32 = start_corner_y + 10;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                // if x as u8 is even, we will fill that square with color
                &x if is_even(x) => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb
                    });
                }
                _ => {
                    // if x is odd, we will fill that square with background color
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb_background
                    });
                }
            } 
        }
    }

    // Create bottom left quadrant
    for (grid_index, grid_element) in intermediate_grid.iter().enumerate() {
        for (i, x) in grid_element.iter().enumerate() {
            let start_corner_x: u32 = (i * 10).try_into().unwrap();
            let start_corner_y: u32 = (240 - (grid_index * 10)).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 10;
            let stop_corner_y: u32 = start_corner_y + 10;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                // if x as u8 is even, we will fill that square with color
                &x if is_even(x) => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb
                    });
                }
                _ => {
                    // if x is odd, we will fill that square with background color
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb_background
                    });
                }
            } 
        }
    }
    
    // Create top right quadrant
    for (grid_index, grid_element) in intermediate_grid.iter().enumerate() {
        for (i, x) in grid_element.iter().enumerate() {
            let start_corner_x: u32 = (240 - (i * 10)).try_into().unwrap();
            let start_corner_y: u32 = (grid_index * 10).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 10;
            let stop_corner_y: u32 = start_corner_y + 10;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                // if x as u8 is even, we will fill that square with color
                &x if is_even(x) => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb
                    });
                }
                _ => {
                    // if x is odd, we will fill that square with background color
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb_background
                    });
                }
            } 
        }
    }

    // Create bottom right quadrant
    for (grid_index, grid_element) in intermediate_grid.iter().enumerate() {
        for (i, x) in grid_element.iter().enumerate() {
            let start_corner_x: u32 = (240 - (i * 10)).try_into().unwrap();
            let start_corner_y: u32 = (240 - (grid_index * 10)).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 10;
            let stop_corner_y: u32 = start_corner_y + 10;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                // if x as u8 is even, we will fill that square with color
                &x if is_even(x) => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb
                    });
                }
                _ => {
                    // if x is odd, we will fill that square with background color
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb_background
                    });
                }
            } 
        }
    }

    let mut fill_center_counter = 0;

    // Fill in middle vertical and horizontal bars
    while fill_center_counter < 25 {
        // fill the vertical bar
        result.push(FillCoordinates {
            start_corner: CornerCoordinates { x: 120, y: fill_center_counter * 10 },
            stop_corner: CornerCoordinates { x: 130, y: fill_center_counter * 10 + 10 },
            rgb: rgb_background
        });

        // fill the horizontal bar
        result.push(FillCoordinates {
            start_corner: CornerCoordinates { x: fill_center_counter * 10, y: 120 },
            stop_corner: CornerCoordinates { x: fill_center_counter * 10 + 10, y: 130 },
            rgb: rgb_background
        });

        fill_center_counter += 1;
    }

    return result;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pixel_map_input() {
        let input: Vec<u8> = vec![55, 70, 56, 51, 66, 49, 54, 53, 55, 70, 70, 49, 70, 67, 53, 51, 66, 57, 50, 68, 67, 49, 56, 49, 52, 56, 65, 49, 68, 54, 53, 68, 70, 67, 50, 68, 52, 66, 49, 70, 65, 51, 68, 54, 55, 55, 50, 56, 52, 65, 68, 68, 68, 50, 48, 48, 49, 50, 54, 68, 57, 48, 54, 57];
        let result = create_pixel_map_input(input);
        assert_eq!(result.len(), 144);
    }
}