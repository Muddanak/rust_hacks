use crate::vectors::vectors::sort_vector_floats;

mod vectors;

fn main() {
    println!("Hello, world!");
    let vv = vec![1.2, 3.4, 2.2, 6.6, 4.4];
    let vv = sort_vector_floats(vv);
    dbg!(vv.clone());
}
