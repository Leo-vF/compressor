use std::env;
use std::time::Instant;

mod files;
mod huffman;
mod huffman_tree;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 4);

    let comp_filepath = &args[2];
    let normal_filepath = &args[3];

    if "d".eq(&args[1]) {
        let comp_file = files::read_comp_file(&comp_filepath);
        let start = Instant::now();
        let decompressed_file = huffman::decompress(comp_file);
        let duration = start.elapsed();
        println!("Time needed for decompression: {:?}", duration);
        files::write_file(&normal_filepath, decompressed_file);
    } else if "c".eq(&args[1]) {
        let decompressed_file = files::read_file(&normal_filepath);
        let start = Instant::now();
        let mut comp_file = huffman::compress(decompressed_file);
        let duration = start.elapsed();
        println!("Time needed for compression: {:?}", duration);
        files::write_comp_file(&comp_filepath, &mut comp_file);
    }
}
