use crate::vectors::vectors::sort_vector_floats;

mod vectors;

fn main() {
    println!("Hello, world!");
    let vv = vec![1.2, 3.4, 2.2, 6.6, 4.4];
    let vv = sort_vector_floats(&vv);

    let test_value: [u8;4] = 0x50450000_u32.to_be_bytes();
    let test_vector: Vec<u8> = vec![0x20, 0x10, 0x50, 0x45, 0x00, 0x00, 0x11, 0x12, 0x12];

    if test_value.iter().all(|item| test_vector.contains(item)) {
        println!("This is the easy way")
    } else {
        println!("No")
    }


    let (test1, test2) = test_vector.iter()
        .enumerate()
        .find(|(_ind, item)| test_value.contains(item) )
        .unwrap();

    println!("{test1} {test2}");
    dbg!(&vv);
}
