use std::env;
use identicon_image_service::{identicon, IdenticonAlgorithm};
use sha2::{Sha256, Digest};

struct IdenticonArgs<'a> {
    text: &'a str,
    algorithm: &'a IdenticonAlgorithm,
    algorithm_file_part: &'a str
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let identicon_args: IdenticonArgs;
    let mut image_text = "Hello World!";
    let mut algorithm = IdenticonAlgorithm::Default;
    let mut algorithm_string = "default";
    
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
                "default" => {
                    algorithm = IdenticonAlgorithm::Default;
                    algorithm_string = &&algorithm_arg;
                },
                "sixty_four_squares" => {
                    algorithm = IdenticonAlgorithm::SixtyFourSquares;
                    algorithm_string = &&algorithm_arg;
                },
                "colorful" => {
                    algorithm = IdenticonAlgorithm::Colorful;
                    algorithm_string = &&algorithm_arg;
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
        algorithm: &algorithm,
        algorithm_file_part: &algorithm_string
    };

    let image_file_name = [identicon_args.text, "_", identicon_args.algorithm_file_part, "_identicon", ".jpg"].concat();
    let input = hash_input(identicon_args.text.to_string());
    let image = identicon(&input, identicon_args.algorithm);
    image.save(image_file_name).expect("Failed to save image.");
}



pub fn hash_input(input: String) -> Vec<u8> {
    // create a Sha256 object
    let mut hasher = Sha256::new();

    // take input test for hashing
    hasher.update(input);

    // get the hashed string by formatting result of hasher finalize
    let result = format!("{:X}", hasher.finalize())
                                .chars()
                                .into_iter()
                                .map(|c| c as u8)
                                .collect();

    return result;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_input() {
        let result: Vec<u8> = hash_input("Hello World!".to_string());
        //println!("{:?}", result);
        assert_eq!(result, [55, 70, 56, 51, 66, 49, 54, 53, 55, 70, 70, 49, 70, 67, 53, 51, 66, 57, 50, 68, 67, 49, 56, 49, 52, 56, 65, 49, 68, 54, 53, 68, 70, 67, 50, 68, 52, 66, 49, 70, 65, 51, 68, 54, 55, 55, 50, 56, 52, 65, 68, 68, 68, 50, 48, 48, 49, 50, 54, 68, 57, 48, 54, 57]);
    }

}