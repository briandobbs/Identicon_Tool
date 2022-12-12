mod identicon;

use identicon::algorithm::Identicon;
use identicon::default_algorithm::DefaultAlgorithm;
use identicon::sixty_four_per_quadrant::SixtyFourSqaresAlgorithm;
use identicon::colorful_algorithm::ColorfulAlgorithm;
use image::RgbImage;

pub enum IdenticonAlgorithm {
    Default,
    Colorful,
    SixtyFourSquares
}


pub fn identicon(input: &Vec<u8>, identicon_algorithm: &IdenticonAlgorithm) -> RgbImage {
    match identicon_algorithm {
        IdenticonAlgorithm::Colorful => {
            let identicon = ColorfulAlgorithm {
                input: input
            };
            identicon.generate()
        },
        IdenticonAlgorithm::SixtyFourSquares => {
            let identicon = SixtyFourSqaresAlgorithm {
                input: input
            };
            identicon.generate()
        },
        IdenticonAlgorithm::Default => {
            let identicon = DefaultAlgorithm {
                input: input
            };
            identicon.generate()
        }
    }
}



