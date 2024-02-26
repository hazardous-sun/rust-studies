use enigo::Enigo;
use enigo::MouseControllable;
use screenshot_rs::screenshot_full;
use std::thread::sleep;
use std::time::Duration;
use image;

fn get_rgb(x: i32, y: i32) {
    let path = String::from("/home/solaire/RustroverProjects/rust-studies/Color picker/color_picker/screenshot.png");
    screenshot_full(path);

    let pixel = image::GenericImageView::get_pixel(path, x as u32, y as u32);
}

fn main() {
    let enigo = Enigo::new();
    while true {
        let (x, y) = enigo.mouse_location();
        println!("x: {x} | y: {y}");
        get_rgb(x, y);
        sleep(Duration::from_secs(2));
    }
}
