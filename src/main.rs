mod building;
mod city;
mod context;
mod math;
mod road;
pub fn main() {
    unsafe {
        raylib::ffi::SetTraceLogLevel(raylib::consts::TraceLogLevel::LOG_ERROR as i32);
        let context = crate::context::Context::new(1000, 1000, 0.875, 0.95, 5);
        let c = city::City::new(1.0, &context);
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
