use std::env;

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
        let decompressed_file = huffman::decompress(comp_file);
        files::write_file(&normal_filepath, decompressed_file);
    } else if "c".eq(&args[1]) {
        let decompressed_file = files::read_file(&normal_filepath);
        let mut comp_file = huffman::compress(decompressed_file);
        files::write_comp_file(&comp_filepath, &mut comp_file);
    }
}
