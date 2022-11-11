use identicon_image_service::create_image;

fn main() {
    let image = create_image();
    image.save("test.jpg").expect("Failed to save image.");
}