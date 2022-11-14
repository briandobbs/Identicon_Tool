use identicon_image_service::identicon;

fn main() {
    let image = identicon("what does this look like?".to_string());
    image.save("test.jpg").expect("Failed to save image.");
}