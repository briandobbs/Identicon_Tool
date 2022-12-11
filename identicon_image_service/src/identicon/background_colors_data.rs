

pub fn get_background_colors_data() -> String {

    let background_colors = r#"
        [
            {
                "name": "gray",
                "rgb": {
                    "r": 158,
                    "g": 158,
                    "b": 158
                }
            },
            {
                "name": "blue gray",
                "rgb": {
                    "r": 96,
                    "g": 125,
                    "b": 139
                }
            }
        ]
    "#;

    return background_colors.to_string();
}