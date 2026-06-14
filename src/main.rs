pub mod utils;

pub mod pool;
pub use raylib::prelude::*;

use crate::pool::Ptr;

pub fn main() {
    let a = new!(10);
    let ptr2 = a;
    let b = *a.lock().unwrap().read();
    *a.lock().unwrap().write() = 15;
    let c = *a.lock().unwrap().read();
    println!("v0:{b}, v1:{c}");
    let d = ptr_cast!(dyn std::fmt::Debug, a);
    let dl = d.lock().unwrap();
    println!("{:#?}", dl.read());
    drop(dl);
    println!("{}", ptr2.lock().unwrap().read());
}
