mod math;
mod road;
mod context;
use crate::math::*;
pub fn main(){
    unsafe{
        rust_raylib::ffi::SetTraceLogLevel(rust_raylib::TraceLogLevel::Error as i32);
        rust_raylib::ffi::InitWindow(1000, 1000, "Hello Sailor\0".as_ptr() as * const i8);
        let context = crate::context::Context::new(1000,1000);
        let r = road::Road::new(&[vec2(250 as f64, 250 as f64), vec2(750 as f64, 750 as f64)]);
        while !rust_raylib::ffi::WindowShouldClose(){
            rust_raylib::ffi::BeginDrawing();
            rust_raylib::ffi::ClearBackground(rust_raylib::color::Color::BLACK.into());
            r.draw(&context);
            rust_raylib::ffi::EndDrawing();
        }
    }
}
