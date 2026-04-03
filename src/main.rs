pub mod array2d;
pub mod col;
pub mod generator;
pub use generator::*;
fn main() {
    //  let x = generate_voronoish(200, 200);
    // render_vector_field_div(&x, "test.png");
    let tmp = generate_city_base(500, 500);
    tmp.render("test.png");
}
