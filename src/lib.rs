mod identicon;

use identicon::algorithm::Identicon;
use identicon::default_algorithm::DefaultAlgorithm;
use image::RgbImage;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IdenticonError {
    details: String
}

impl IdenticonError {
    fn new(msg: &str) -> IdenticonError {
        IdenticonError{details: msg.to_string()}
    }
}

impl fmt::Display for IdenticonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for IdenticonError {
    fn description(&self) -> &str {
        &self.details
    }
}


pub fn identicon(input: String, identicon_algorithm: &str) -> Result<RgbImage, IdenticonError> {
    match identicon_algorithm {
        "default" => Ok({
            let identicon = DefaultAlgorithm {
                input: input.to_string()
            };
            identicon.generate()
        }),
        _ => Err(IdenticonError::new("Identicon algorithm provided is not one of the algorithms available."))
    }
}

//fn pick_color()



