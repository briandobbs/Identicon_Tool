

pub fn get_colors_data() -> String {

    let colors = r#"
        [
            {
                "name": "red",
                "rgb": {
                    "r": 244,
                    "g": 67,
                    "b": 54
                }
                
            },
            {
                "name": "pink",
                "rgb": {
                    "r": 232,
                    "g": 30,
                    "b": 99
                }
                
            },
            {
                "name": "purple",
                "rgb": {
                    "r": 156,
                    "g": 39,
                    "b": 176
                }
                
            },
            {
                "name": "deep purple",
                "rgb": {
                    "r": 103,
                    "g": 58,
                    "b": 183
                }
                
            },
            {
                "name": "indigo",
                "rgb": {
                    "r": 63,
                    "g": 81,
                    "b": 181
                }
                
            },
            {
                "name": "blue",
                "rgb": {
                    "r": 33,
                    "g": 150,
                    "b": 243
                }
                
            },
            {
                "name": "light blue",
                "rgb": {
                    "r": 3,
                    "g": 169,
                    "b": 244
                }
                
            },
            {
                "name": "cyan",
                "rgb": {
                    "r": 0,
                    "g": 188,
                    "b": 212
                }
                
            },
            {
                "name": "teal",
                "rgb": {
                    "r": 0,
                    "g": 150,
                    "b": 136
                }
                
            },
            {
                "name": "green",
                "rgb": {
                    "r": 76,
                    "g": 175,
                    "b": 80
                }
                
            },
            {
                "name": "light green",
                "rgb": {
                    "r": 139,
                    "g": 195,
                    "b": 74
                }
                
            },
            {
                "name": "lime",
                "rgb": {
                    "r": 205,
                    "g": 220,
                    "b": 57
                }
                
            },
            {
                "name": "yellow",
                "rgb": {
                    "r": 255,
                    "g": 235,
                    "b": 59
                }
                
            },
            {
                "name": "amber",
                "rgb": {
                    "r": 255,
                    "g": 193,
                    "b": 7
                }
                
            },
            {
                "name": "orange",
                "rgb": {
                    "r": 255,
                    "g": 152,
                    "b": 0
                }
                
            },
            {
                "name": "deep orange",
                "rgb": {
                    "r": 255,
                    "g": 87,
                    "b": 34
                }
                
            },
            {
                "name": "brown",
                "rgb": {
                    "r": 121,
                    "g": 85,
                    "b": 72
                }
                
            }
        ]
        "#;
    return colors.to_string();
}