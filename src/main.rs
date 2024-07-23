use prof::private_profile_start;
mod building;
mod city;
mod context;
mod math;
mod prof;
mod road;
mod water;
use crate::context::Context;
pub fn main() {
    let t = private_profile_start("main");
    let context = Context::new(1000, 1000, 0.85, 0.85, 1.0, 1.0, 1.0, 8.0);
    let c = city::City::new(2.0, &context);
    println!("profile finished\n");
    drop(t);
    unsafe {
        raylib::ffi::SetTraceLogLevel(raylib::consts::TraceLogLevel::LOG_ERROR as i32);
        raylib::ffi::InitWindow(1000, 1000, "Hello Sailor\0".as_ptr() as *const i8);
        let tex = raylib::ffi::LoadRenderTexture(1000, 1000);
        raylib::ffi::BeginTextureMode(tex.clone());
        raylib::ffi::ClearBackground(raylib::color::Color::WHITE.into());
        c.draw(&context);
        raylib::ffi::EndTextureMode();
        while !raylib::ffi::WindowShouldClose() {
            raylib::ffi::BeginDrawing();
            raylib::ffi::ClearBackground(raylib::color::Color::WHITE.into());
            raylib::ffi::DrawTexture(
                tex.clone().texture,
                0,
                0,
                raylib::color::Color::WHITE.into(),
            );
            raylib::ffi::EndDrawing();
        }
    }
}
