pub mod utils;
pub use raylib::prelude::*;
pub fn main() {
    let mut img = Image::gen_image_color(100, 100, Color::WHITE);
    utils::draw_text_to_image(&mut img, "test", 10, 10, 16, Color::BLACK);
    img.export_image("test2.png");
}
