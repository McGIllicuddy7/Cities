use crate::{buildings::generate_voronoi, utils::Arena};

pub mod buildings;
pub mod city;
pub mod utils;
pub fn main() {
    let ar = Arena::new();
    let buf = ar.alloc_buffer(50);
    for i in 0..buf.len() {
        buf[i] = i as i32;
    }
    let bf2 = ar.alloc_buffer(50);
    for i in 0..buf.len() {
        bf2[i] = i as i32 + 100;
    }
    let bf3 = ar.concat_buffers(buf, bf2);
    let st = ar.concat_strs("hello ", "world!");
    println!("{:#?}", bf3);
    println!("{}", st);
}
