mod building;
mod city;
mod context;
mod math;
mod road;
pub fn main() {
    let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).blocklist(&["libc", "libgcc", "pthread", "vdso"]).build().unwrap();
    let context = crate::context::Context::new(1000, 1000, 0.85, 0.95, 5, 2.0, 8.0,12.0);
    let _c = city::City::new(0.5, &context);
    if let Ok(report) = guard.report().build() {
        println!("report: {:?}", &report);
    };
    /*
   unsafe{
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
    }*/
}
