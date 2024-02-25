use enigo;
use enigo::MouseControllable;
use screenshot;
use image::{GenericImageView, Rgba};

fn get_rgb_value(x: u32, y: u32) -> Option<Rgba<u8>> {
    let screenshot = screenshot::Screenshot::get_pixel(x, y);
}

fn main() {
    let enigo = enigo::Enigo::new();
    while true {
        let (x, y) = enigo.mouse_location();
        println!("x: {x} | y: {y}");
    }
}
