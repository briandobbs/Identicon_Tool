use hex_color::HexColor;
use image::{RgbImage, Rgb};

use super::algorithm::Identicon;
use super::utils::hash_input;

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

pub struct DefaultAlgorithm {
    pub input: String
}

impl Identicon for DefaultAlgorithm {
    
    fn generate(&self) -> RgbImage {
        let pixel_map_input = create_pixel_map_input(self.input.to_string());

        let pixel_map = build_pixel_map(pixel_map_input);

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

fn create_pixel_map_input(input: String) -> Vec<char> {
    let hash_result_1: Vec<char> = hash_input(input).chars().collect();
    let mut result: Vec<char> = hash_result_1.clone();
    result.extend(hash_result_1.clone());
    result.extend(hash_result_1);
    result.truncate(144);

    return result;
}

fn build_pixel_map(input: Vec<char>) -> Vec<FillCoordinates> {
    let mut result = Vec::new();

    let intermediate_grid: Vec<&[char]> = input.chunks(12).collect();

    let hex_code: String = input.iter().take(6).collect();

    let hex_string = ["#", &hex_code].concat();

    let hex = HexColor::parse_rgb(&hex_string).unwrap();

    let rgb = RgbValues {
        r: hex.r,
        g: hex.g,
        b: hex.b,
    };

    let rgb_background = RgbValues {
        r: 96,
        g: 126,
        b: 139,
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
                &x if x.is_numeric() && x as u8 <= 127 => {
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
            let start_corner_x: u32 = (i * 10).try_into().unwrap();
            let start_corner_y: u32 = (240 - (grid_index * 10)).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 10;
            let stop_corner_y: u32 = start_corner_y + 10;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                &x if x.is_numeric() && x as u8 <= 127 => {
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
            let start_corner_x: u32 = (240 - (i * 10)).try_into().unwrap();
            let start_corner_y: u32 = (grid_index * 10).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 10;
            let stop_corner_y: u32 = start_corner_y + 10;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                &x if x.is_numeric() && x as u8 <= 127 => {
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
            let start_corner_x: u32 = (240 - (i * 10)).try_into().unwrap();
            let start_corner_y: u32 = (240 - (grid_index * 10)).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 10;
            let stop_corner_y: u32 = start_corner_y + 10;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                &x if x.is_numeric() && x as u8 <= 127 => {
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
        let result = create_pixel_map_input("Typical Name".to_string());
        assert_eq!(result.len(), 144);
    }

    #[test]
    fn test_u8_comparison() {
        let result: String = hash_input("Hello World!".to_string());
        println!("Check value: {}", result.chars().nth(3).unwrap());
        println!("Check 4: {}", 4 as char);
        assert!((result.chars().nth(3).unwrap() as u8) < 250);
    }

}