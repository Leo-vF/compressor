use std::fs;

#[derive(Debug)]
pub struct Count {
    pub char: u8,
    pub count: u32,
}

#[derive(Debug)]
pub struct HuffmanMapping {
    pub char: u8,
    pub len_of_encoding: u8,
    pub encoding: Vec<u8>,
}

#[derive(Debug)]
pub struct File {
    pub mappings: Vec<HuffmanMapping>,
    pub data: Vec<u8>,
}

pub fn read_comp_file(path: &str) -> File {
    let mut file: Vec<u8> = fs::read(path).unwrap();
    let mut n_mappings = file.remove(0);
    let mut mappings = Vec::new();
    while n_mappings > 0 {
        let char = file.remove(0);
        let len_of_encoding = file.remove(0);
        let mut encoding = Vec::new();
        for _i in 0..len_of_encoding {
            encoding.push(file.remove(0));
        }
        mappings.push(HuffmanMapping {
            char,
            len_of_encoding,
            encoding,
        });
        n_mappings -= 1;
    }
    File {
        mappings,
        data: file,
    }
}

pub fn read_file(path: &str) -> Vec<u8> {
    fs::read(path).unwrap()
}

pub fn write_comp_file(path: &str, file: File) {
    // TODO: write number of counts as u8 in first byte of the file
    // TODO: assemble u8 1's and 0's back to actual u8's to save space
}

pub fn write_file(path: &str, chars: Vec<u8>) {
    fs::write(path, chars).unwrap();
}
