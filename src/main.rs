pub mod utils;

pub mod pool;
pub use raylib::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::pool::{Ptr, save_pool_state};
make_pooled!(i32, 10000);
pub fn main() {
    register_types!(i32);
    let a = new!(10);
    (0..1).into_par_iter().for_each(move |tid| {
        println!("{}", tid);
    });
    println!("{:#?}", a.lock().unwrap().read());
}
