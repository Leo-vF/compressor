use crate::huffman::*;

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

    let tree = create_huffman_tree_from_bytestream(&test);
    let tmp = get_hashmap_for_compression(Box::from(tree));
    println!("{}❤️", tmp.len());

}
