use identicon_image_service::identicon;

fn main() {
    let image = identicon("hola!".to_string());
    image.save("test.jpg").expect("Failed to save image.");
}