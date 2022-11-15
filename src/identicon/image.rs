

pub fn create_image(pixel_map: Vec<FillCoordinates>) -> RgbImage {
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