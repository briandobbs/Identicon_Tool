use image::{RgbImage, Rgb};
use sha2::{Sha256, Sha512, Digest};

//struct FillStartStop(u32, u32);

struct FillCoordinates {
    start_corner: CornerCoordinates,
    stop_corner: CornerCoordinates
}

struct CornerCoordinates {
    x: u32,
    y: u32
}

pub fn create_image() -> RgbImage {
    let mut img = RgbImage::new(250, 250);

    let pixel_map: [FillCoordinates; 5] = [
        FillCoordinates {
            start_corner: CornerCoordinates { x: 0, y: 0 },
            stop_corner: CornerCoordinates { x: 10, y: 10 }
        },
        FillCoordinates {
            start_corner: CornerCoordinates { x: 20, y: 20 },
            stop_corner: CornerCoordinates { x: 30, y: 30 }
        },
        FillCoordinates {
            start_corner: CornerCoordinates { x: 140, y: 140 },
            stop_corner: CornerCoordinates { x: 150, y: 150 }
        },
        FillCoordinates {
            start_corner: CornerCoordinates { x: 200, y: 200 },
            stop_corner: CornerCoordinates { x: 210, y: 210 }
        },
        FillCoordinates {
            start_corner: CornerCoordinates { x: 60, y: 60 },
            stop_corner: CornerCoordinates { x: 70, y: 70 }
        }
    ];
    
    for fill in pixel_map.iter() {
        for x in fill.start_corner.x..fill.stop_corner.x {
            for y in fill.start_corner.y..fill.stop_corner.y  {
                img.put_pixel(x, y, Rgb([255, 0, 0]));
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

pub fn hash_input_512(input: String) -> String {
    // create a Sha256 object
    let mut hasher = Sha512::new();

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
    fn test_hash_input_512() {
        let result: String = hash_input_512("Hello World!".to_string());
        assert_eq!(result, "861844D6704E8573FEC34D967E20BCFEF3D424CF48BE04E6DC08F2BD58C729743371015EAD891CC3CF1C9D34B49264B510751B1FF9E537937BC46B5D6FF4ECC8");
    }
}
