use image::{RgbImage, Rgb};
use sha2::{Sha256, Digest};

pub fn create_image() -> RgbImage {
    let mut img = RgbImage::new(32, 32);

    for x in 15..=17 {
        for y in 8..24 {
            img.put_pixel(x, y, Rgb([255, 0, 0]));
            img.put_pixel(y, x, Rgb([255, 0, 0]));
        }
    }

    img
}

pub fn hash_input(input: String) -> String {
    let mut hasher = Sha256::new();

    hasher.update(input);

    let result: String = format!("{:X}", hasher.finalize());

    return result;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_hash_input() {
        let result: String = hash_input("Hello World!".to_string());
        println!("{}", result);
    }
}
