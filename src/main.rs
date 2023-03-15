use crate::huffman::*;

mod files;
mod huffman_tree;
mod huffman;

fn main() {
    println!("Hello, world!");
    let mut test_0: Vec<u8> = Vec::new();
    test_0.push(12);
    test_0.push(12);
    test_0.push(12);
    test_0.push(12);
    test_0.push(42);
    test_0.push(42);
    test_0.push(12);
    test_0.push(42);
    test_0.push(23);
    test_0.push(21);

    let tree = create_huffman_tree_from_bytestream(&test_0);
    let test_1 = get_hashmap_for_compression(Box::from(tree));
    println!("{}❤️", test_1.len());
    let test_2 = get_huffman_code_header_for_file(test_1);
    println!("{:?}", test_2);
    let test_3 = get_hashmap_for_decompression(test_2);
    println!("{}❤️", test_3.len())
}
