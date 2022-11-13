use image::{RgbImage, Rgb};
use sha2::{Sha256, Digest};

//struct FillStartStop(u32, u32);

pub struct FillCoordinates {
    pub start_corner: CornerCoordinates,
    pub stop_corner: CornerCoordinates,
    pub fill: bool
}

pub struct CornerCoordinates {
    pub x: u32,
    pub y: u32
}

pub fn identicon(input: String) -> RgbImage {
    let pixel_map_input = create_pixel_map_input(input);

    let pixel_map = build_pixel_map(pixel_map_input);

    let image = create_image(pixel_map);

    return image;
}

pub fn create_image(pixel_map: Vec<FillCoordinates>) -> RgbImage {
    let mut img = RgbImage::new(250, 250);

    
    for fill in pixel_map.iter() {
        if fill.fill {
            for x in fill.start_corner.x..fill.stop_corner.x {
                for y in fill.start_corner.y..fill.stop_corner.y  {
                    img.put_pixel(x, y, Rgb([255, 0, 0]));
                }
            }
        }
        
    }

    img
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
                    println!("We got a number! {}, {}", i, x);
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        fill: false
                    });
                }
                _ => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        fill: true
                    });
                }
            } 
        };
    }
    
    

    return result;
}

fn hash_input(input: String) -> String {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // take input test for hashing
    hasher.update(input);

    // get the hashed string by formatting result of hasher finalize
    let result: String = format!("{:X}", hasher.finalize());

    return result;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_input() {
        let result: String = hash_input("Hello World!".to_string());
        assert_eq!(result, "7F83B1657FF1FC53B92DC18148A1D65DFC2D4B1FA3D677284ADDD200126D9069");
    }

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
