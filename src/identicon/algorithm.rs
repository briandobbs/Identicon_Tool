

fn create_pixel_map_input(input: String) -> Vec<char> {
    let hash_result_1: Vec<char> = hash_input(input).chars().collect();
    let mut result: Vec<char> = hash_result_1.clone();
    result.extend(hash_result_1.clone());
    result.extend(hash_result_1);
    result.truncate(144);

    return result;
}

fn build_pixel_map(input: Vec<char>) -> Vec<FillCoordinates> {
    let mut result = Vec::new();

    let intermediate_grid: Vec<&[char]> = input.chunks(12).collect();

    let hex_code: String = input.iter().take(6).collect();

    let hex_string = ["#", &hex_code].concat();

    let hex = HexColor::parse_rgb(&hex_string).unwrap();

    let rgb = RgbValues {
        r: hex.r,
        g: hex.g,
        b: hex.b,
    };

    let rgb_background = RgbValues {
        r: 117,
        g: 117,
        b: 117,
    };
        

    // Create top left quadrant
    for (grid_index, grid_element) in intermediate_grid.iter().enumerate() {
        for (i, x) in grid_element.iter().enumerate() {
            let start_corner_x: u32 = (i * 10).try_into().unwrap();
            let start_corner_y: u32 = (grid_index * 10).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 10;
            let stop_corner_y: u32 = start_corner_y + 10;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                &x if x.is_numeric() && x as u8 <= 127 => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb_background
                    });
                }
                _ => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb
                    });
                }
            } 
        }
    }

    // Create bottom left quadrant
    for (grid_index, grid_element) in intermediate_grid.iter().enumerate() {
        for (i, x) in grid_element.iter().enumerate() {
            let start_corner_x: u32 = (i * 10).try_into().unwrap();
            let start_corner_y: u32 = (240 - (grid_index * 10)).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 10;
            let stop_corner_y: u32 = start_corner_y + 10;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                &x if x.is_numeric() && x as u8 <= 127 => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb_background
                    });
                }
                _ => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb
                    });
                }
            } 
        }
    }
    
    // Create top right quadrant
    for (grid_index, grid_element) in intermediate_grid.iter().enumerate() {
        for (i, x) in grid_element.iter().enumerate() {
            let start_corner_x: u32 = (240 - (i * 10)).try_into().unwrap();
            let start_corner_y: u32 = (grid_index * 10).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 10;
            let stop_corner_y: u32 = start_corner_y + 10;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                &x if x.is_numeric() && x as u8 <= 127 => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb_background
                    });
                }
                _ => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb
                    });
                }
            } 
        }
    }

    // Create bottom right quadrant
    for (grid_index, grid_element) in intermediate_grid.iter().enumerate() {
        for (i, x) in grid_element.iter().enumerate() {
            let start_corner_x: u32 = (240 - (i * 10)).try_into().unwrap();
            let start_corner_y: u32 = (240 - (grid_index * 10)).try_into().unwrap();
            let stop_corner_x: u32 = start_corner_x + 10;
            let stop_corner_y: u32 = start_corner_y + 10;
            let start_corner_coordinates = CornerCoordinates { x: start_corner_x, y: start_corner_y };
            let stop_corner_coordinates = CornerCoordinates { x: stop_corner_x, y: stop_corner_y };
            match x {
                &x if x.is_numeric() && x as u8 <= 127 => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb_background
                    });
                }
                _ => {
                    result.push(FillCoordinates {
                        start_corner: start_corner_coordinates,
                        stop_corner: stop_corner_coordinates,
                        rgb: rgb
                    });
                }
            } 
        }
    }

    let mut fill_center_counter = 0;

    // Fill in middle vertical and horizontal bars
    while fill_center_counter < 25 {
        // fill the vertical bar
        result.push(FillCoordinates {
            start_corner: CornerCoordinates { x: 120, y: fill_center_counter * 10 },
            stop_corner: CornerCoordinates { x: 130, y: fill_center_counter * 10 + 10 },
            rgb: rgb_background
        });

        // fill the horizontal bar
        result.push(FillCoordinates {
            start_corner: CornerCoordinates { x: fill_center_counter * 10, y: 120 },
            stop_corner: CornerCoordinates { x: fill_center_counter * 10 + 10, y: 130 },
            rgb: rgb_background
        });

        fill_center_counter += 1;
    }

    return result;
}