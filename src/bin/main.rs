use identicon_image_service::identicon;

fn main() {

    // let mut pixel_map: Vec<FillCoordinates> = Vec::new();
    
    // pixel_map.push(FillCoordinates {
    //     start_corner: CornerCoordinates { x: 0, y: 0 },
    //     stop_corner: CornerCoordinates { x: 10, y: 10 },
    //     fill: true
    // });

    // pixel_map.push(FillCoordinates {
    //     start_corner: CornerCoordinates { x: 10, y: 10 },
    //     stop_corner: CornerCoordinates { x: 20, y: 20 },
    //     fill: false
    // });
        
    // pixel_map.push(FillCoordinates {
    //     start_corner: CornerCoordinates { x: 20, y: 20 },
    //     stop_corner: CornerCoordinates { x: 30, y: 30 },
    //     fill: true
    // });
        
    

    // let image = create_image(pixel_map);
    let image = identicon("Hello World!".to_string());
    image.save("test.jpg").expect("Failed to save image.");
}