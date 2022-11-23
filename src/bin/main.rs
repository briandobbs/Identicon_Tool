use std::env;
use identicon_image_service::identicon;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_text = &args[1];
    let image_file_name = [image_text, "_identicon", ".jpg"].concat();
    let image = identicon(image_text.to_string(), "default");
    //image.save(image_file_name).expect("Failed to save image.");
    match image {
        Ok(i) => i.save(image_file_name).expect("Failed to save image."),
        Err(e) => println!("Error getting the identicon: {e:?}")
    }
}