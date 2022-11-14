use std::env;
use identicon_image_service::identicon;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_text = &args[1];
    let image_file_name = [image_text, "_identicon", ".jpg"].concat();
    let image = identicon(image_text.to_string());
    image.save(image_file_name).expect("Failed to save image.");
}