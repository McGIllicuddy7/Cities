use rust_raylib::ffi::*;
mod road;
fn main() {
    unsafe {
        SetTraceLogLevel(rust_raylib::TraceLogLevel::Error as i32);
        InitWindow(1000, 1000, "Hello Sailor\0".as_ptr() as *const i8);
        let l = road::LineSpline::new(&[
            Vector2 {
                x: 0 as f32,
                y: 0 as f32,
            }
            .into(),
            Vector2 {
                x: 1000 as f32,
                y: 1000 as f32,
            }
            .into(),
        ]);
        while !WindowShouldClose() {
            BeginDrawing();
            ClearBackground(rust_raylib::color::Color::PINK.into());
            l.draw();
            EndDrawing();
        }
    }
}
