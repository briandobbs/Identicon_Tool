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

pub fn hash_input(input: String) -> String {
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

}
