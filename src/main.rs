pub mod utils;

pub mod pool;
pub use raylib::prelude::*;

use crate::pool::Ptr;

pub fn main() {
    let a = Ptr::new(10);
    let b = *a.lock().unwrap().read();
    *a.lock().unwrap().write() = 15;
    let c = *a.lock().unwrap().read();
    println!("v0:{b}, v1:{c}");
}
