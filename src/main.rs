use crate::huffman::{create_huffman_tree, create_huffman_tree_from_bytestream};

mod files;
mod huffman_tree;
mod huffman;

fn main() {
    println!("Hello, world!");
    let mut test: Vec<u8> = Vec::new();
    test.push(12);
    test.push(12);
    test.push(12);
    test.push(12);
    test.push(42);
    test.push(42);
    test.push(12);
    test.push(42);
    test.push(23);
    test.push(21);

    create_huffman_tree_from_bytestream(test);
}
