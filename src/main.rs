use crate::huffman::*;

mod files;
mod huffman_tree;
mod huffman;

fn main() {
    let file: Vec<u8> = vec![12, 12, 12, 12, 42, 42, 12, 42, 23, 21];
    println!("{:?}", file);
    let compressed_file = compress(file);
    println!("{:?}", compressed_file.data);
    let decompressed_file = decompress(compressed_file);
    println!("{:?}", decompressed_file);
}
