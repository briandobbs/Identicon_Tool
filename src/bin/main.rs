use std::env;
use identicon_image_service::identicon;

struct IdenticonArgs<'a> {
    text: &'a str,
    algorithm: &'a str
}


fn main() {
    let args: Vec<String> = env::args().collect();
    // let image_text = &args[1];
    let identicon_args: IdenticonArgs;
    let mut image_text = "Hello World!";
    let mut algorithm = "default";
    
    match args.len() {
        // no arguments passed
        1 => {
            println!("No arguments were passed. I've generated an identicon for you with the text 'Hello World!' and using the default algorithm.");
            
        },
        // one argument passed
        2 => {
           image_text = &args[1];
            println!("One argument was passed. I've generated an identicon for you with the text '{}' and using the default algorithm.", image_text);
            
        },
        3 => {
           image_text = &args[1];
            let algorithm_arg = &args[2];
            match &algorithm_arg[..] {
                "default" | "sixty_four_squares" => {
                    algorithm = &algorithm_arg;
                },
                _ => {
                    println!("The value {} did not match any of the available identicon algorithms. The default algorithm has been used instead.", algorithm_arg);
                    
                }
            }
        }
        _ => {
            println!("Something went wrong. Please try again.");
            return;
        }
    }

    identicon_args = IdenticonArgs {
        text: &image_text,
        algorithm: &algorithm
    };

    let image_file_name = [identicon_args.text, "_", identicon_args.algorithm, "_identicon", ".jpg"].concat();
    let image = identicon(identicon_args.text, identicon_args.algorithm);

    match image {
        Ok(i) => i.save(image_file_name).expect("Failed to save image."),
        Err(e) => println!("Error getting the identicon: {e:?}")
    }
}