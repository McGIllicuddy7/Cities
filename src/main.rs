pub mod utils;

pub use raylib::prelude::*;

use crate::utils::DrawCPUText;
pub fn main() {
    for i in 0..=614 {
        let mut img = Image::gen_image_color(1000, 1000, Color::WHITE);
        img.draw_line(0, 100, 1000, 100, Color::BLACK);
        img.draw_line(0, 132, 1000, 132, Color::BLACK);
        img.draw_text_cpu(
        "hiiii testing 1 2 3: a b c d e f g h i j k l m n o p q r s t u v w x y z\nA B C D E F G H I J K L M N O P Q R S T U V W X Y Z , : ; | 1 2 3 4 5 6 7 8 9 0",
        0,
        100,
        16,
        Color::GREEN,
    );
        let b = utils::Boundary {
            center_x: 500,
            center_y: 500,
            width: 100,
            height: 100,
            rotation: (i as f32 / 100.),
        };
        b.draw_to_image(&mut img, Color::RED);
        img.export_image("test2.png");
    }
}
