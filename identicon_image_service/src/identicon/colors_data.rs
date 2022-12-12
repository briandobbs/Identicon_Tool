use super::color::Rgb;

pub fn get_color(input: &u8) -> Rgb {
    let index = input % 17;
    match  index {
        // "red"
        0 => return Rgb {
            r: 244,
            g: 67,
            b: 54
        },
        // "pink"
        1 => return Rgb {
            r: 232,
            g: 30,
            b: 99
        },
        // "purple"
        2 => return Rgb {
            r: 156,
            g: 39,
            b: 176
        },
        // "deep purple"
        3 => return Rgb {
            r: 103,
            g: 58,
            b: 183
        },
        // "indigo"
        4 => return Rgb {
            r: 63,
            g: 81,
            b: 181
        },
        // "blue"
        5 => return Rgb {
            r: 33,
            g: 150,
            b: 243
        },
        // "light blue"
        6 => return Rgb {
            r: 3,
            g: 169,
            b: 244
        },
        // "cyan"
        7 => return Rgb {
            r: 0,
            g: 188,
            b: 212
        },
        // "teal"
        8 => return Rgb {
            r: 0,
            g: 150,
            b: 136
        },
        // "green"
        9 => return Rgb {
            r: 76,
            g: 175,
            b: 80
        },
        // "light green"
        10 => return Rgb {
            r: 139,
            g: 195,
            b: 74
        },
        // "lime"
        11 => return Rgb {
            r: 205,
            g: 220,
            b: 57
        },
        // "yellow"
        12 => return Rgb {
            r: 255,
            g: 235,
            b: 59
        },
        // "amber"
        13 => return Rgb {
            r: 255,
            g: 193,
            b: 7
        },
        // "orange"
        14 => return Rgb {
            r: 255,
            g: 152,
            b: 0
        },
        // "deep orange"
        15 => return Rgb {
            r: 255,
            g: 87,
            b: 34
        },
        // "brown"
        16 => return Rgb {
            r: 121,
            g: 85,
            b: 72
        },
        // assign "brown" to case that falls outside 0 - 16 index range, even though this will never happen
        _ => return Rgb {
            r: 121,
            g: 85,
            b: 72
        }
    }
}