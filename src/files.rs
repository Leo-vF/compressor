use std::fs;

struct Count {
    char: u8,
    count: u32,
}

enum Filetype {
    Compressed,
    Uncompressed,
}

struct File {
    filetype: Filetype,
    probs: Vec<Count>,
    data: Vec<u8>,
}

pub fn read_comp_file(path: &str) -> File {}

pub fn read_file(path: &str) -> File {
    // TODO: read file as vec of u8's
    // count number of appearances save to vec of count's
    // make File struct
}

pub fn write_comp_file(path: &str, file: File) {
    // TODO: assemble u8 1's and 0's back to actual u8's to save space
    // TODO: write Counts then ETX then previously assembled u8's
}

pub fn write_file(path: &str, chars: Vec<u8>) {
    fs::write(path, chars);
}
