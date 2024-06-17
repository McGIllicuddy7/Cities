mod math;
mod road;
mod context;
use crate::math::*;
pub fn main(){
    unsafe{
        //raylib::ffi::SetTraceLogLevel(raylib::consts::TraceLogLevel::LOG_ERROR as i32);
        raylib::ffi::InitWindow(1000, 1000, "Hello Sailor\0".as_ptr() as * const i8);
        let context = crate::context::Context::new(1000,1000);
        let r = road::Road::new(&[vec2(250 as f64, 250 as f64), vec2(750 as f64, 750 as f64)]);
        while !raylib::ffi::WindowShouldClose(){
            raylib::ffi::BeginDrawing();
            raylib::ffi::ClearBackground(raylib::color::Color::BLACK.into());
            r.draw(&context);
            raylib::ffi::EndDrawing();
        }
    }
}
