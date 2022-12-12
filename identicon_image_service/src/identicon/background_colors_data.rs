use super::color::Rgb;

pub fn get_background_color(index: &u8) -> Rgb {
    if let 0 = index % 2 {
        // result is even
        return Rgb {
            r: 158,
            g: 158,
            b: 158
        };
    } else {
        // result is odd
        return Rgb {
            r: 96,
            g: 125,
            b: 139
        };
    };
}