use image::RgbImage;

pub trait Identicon {
    fn generate(&self) -> RgbImage;
}