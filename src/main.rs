extern crate ring;

use ring::digest::{digest, Algorithm, SHA256};

fn main() {
    let s: Vec<u8> = vec![1, 2, 3];
    let x = digest(&SHA256, &s);
    println!("{:?}", x);
}

#[test]
fn main_test() {
    assert!(true);
    assert!(false);
}
