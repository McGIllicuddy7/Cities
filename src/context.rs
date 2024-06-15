pub struct Context{
    pub height:i32,
    pub width:i32,
    pub street_shader:rust_raylib::ffi::Shader,
    pub map_mesh:rust_raylib::ffi::Mesh,
    pub cam:rust_raylib::ffi::Camera,
}
impl Context{
    pub fn new(height:i32, width:i32)->Self{
        let street_shader = unsafe{
            rust_raylib::ffi::LoadShader("street.vs\0".as_ptr() as *const i8,"street.fs\0".as_ptr() as *const i8)
        };
        let map_mesh = unsafe{
            rust_raylib::ffi::GenMeshPlane(height as f32, width as f32, 1, 1)
        };
        let cam = rust_raylib::ffi::Camera{fovy:90.0, position:rust_raylib::ffi::Vector3{x:0.0, y:0.0, z:100.0},projection:rust_raylib::ffi::CameraProjection::Orthographic as i32, target:rust_raylib::ffi::Vector3{x:0.0, y:0.0, z:-1.0}, up:rust_raylib::ffi::Vector3{x:0.0, y:0.0, z:1.0}};
        return Self{height:height, width:width, street_shader:street_shader, map_mesh:map_mesh, cam:cam};
    }
}
impl Drop for Context{
    fn drop(&mut self) {
        unsafe{
            rust_raylib::ffi::UnloadShader(self.street_shader.clone());
            rust_raylib::ffi::UnloadMesh(self.map_mesh.clone());
        }
    }
}