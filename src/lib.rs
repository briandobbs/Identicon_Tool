use image::{RgbImage, Rgb};
use sha2::{Sha256, Digest};

struct FillStartStop(u32, u32);

pub fn create_image() -> RgbImage {
    let mut img = RgbImage::new(250, 250);

    let pixel_map: [FillStartStop; 5] = [
        FillStartStop(0, 50),
        FillStartStop(65, 75),
        FillStartStop(0, 50),
        FillStartStop(0, 50),
        FillStartStop(0, 50)
    ];

    for x in 0..=20 {
        for y in 0..10 {
            img.put_pixel(x, y, Rgb([255, 0, 0]));
            img.put_pixel(y, x, Rgb([255, 0, 0]));
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
