
pub struct Context{
    pub height:i32,
    pub width:i32,
}
impl Context{

    pub fn new(height:i32, width:i32)->Self{
        unsafe{
            raylib::ffi::SetRandomSeed(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32);
        }
        return Self{height:height, width:width};
    }
}
impl Drop for Context{
    fn drop(&mut self) {

    }
}