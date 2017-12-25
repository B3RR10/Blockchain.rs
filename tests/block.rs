extern crate blockchain;

use std::fmt::Write;
use blockchain::block::*;

#[test]
fn create_genesis_block() {
    let block = Block::new_genesis_block();

    println!("{}", block);
}

#[test]
fn create_new_block() {
    let block = Block::new("MyData", Block::new_genesis_block().hash);

    println!("{}", block);
}
