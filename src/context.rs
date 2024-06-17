
pub struct Context{
    pub height:i32,
    pub width:i32,
    pub street_shader:raylib::ffi::Shader,
    pub street_mat:raylib::ffi::Material,
    pub map_mesh:raylib::ffi::Mesh,
    pub cam:raylib::ffi::Camera,
    pub street_location:raylib::ffi::Matrix,
}
impl Context{
    pub fn new(height:i32, width:i32)->Self{
        let street_shader = unsafe{
            raylib::ffi::LoadShader("src/street.vs\0".as_ptr() as *const i8,"src/street.fs\0".as_ptr() as *const i8)
        };
        let map_mesh = unsafe{
            raylib::ffi::GenMeshCube(10 as f32,10 as f32, 10 as f32)
        };
        let cam = raylib::ffi::Camera{fovy:90.0, position:raylib::ffi::Vector3{x:0.0, y:0.0, z:-10.0},projection:raylib::consts::CameraProjection::CAMERA_PERSPECTIVE as i32, target:raylib::ffi::Vector3{x:0.0, y:0.0, z:1.0}, up:raylib::ffi::Vector3{x:0.0, y:0.0, z:1.0}};
        let mut mat:raylib::ffi::Material = unsafe{raylib::ffi::LoadMaterialDefault()};
        mat.shader = street_shader.clone();
        return Self{height:height, width:width, street_shader:street_shader, map_mesh:map_mesh, street_mat:mat,cam:cam, street_location: raylib::math::Matrix::translate(0 as f32, 0 as f32, 0 as f32).into()};
    }
}
impl Drop for Context{
    fn drop(&mut self) {
        unsafe{
            raylib::ffi::UnloadMesh(self.map_mesh.clone());
            raylib::ffi::UnloadMaterial(self.street_mat.clone());
        }
    }
}