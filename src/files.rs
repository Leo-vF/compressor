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
    pub last_byte_offset: u8,
    pub mappings: Vec<HuffmanMapping>,
    pub data: Vec<u8>,
}

impl HuffmanMapping {
    pub fn new(char: u8, len_of_encoding: u8, encoding: Vec<u8>) -> Self {
        Self {
            char,
            len_of_encoding,
            encoding,
        }
    }

    fn linearize(&mut self) -> Vec<u8> {
        let mut result = Vec::new();
        result.push(self.char);
        result.push(self.len_of_encoding);
        result.append(&mut self.encoding);
        return result;
    }
}

pub fn read_comp_file(path: &str) -> File {
    let mut file: Vec<u8> = fs::read(path).unwrap();
    let mut n_mappings = file.remove(0);
    let last_byte_offset = file.remove(0);
    let mut mappings = Vec::new();
    while n_mappings > 0 {
        let char = file.remove(0);
        // this next line converts the len of encoding in bit size
        // to byte size via ceiling divsion by 8
        let len_of_encoding = file.remove(0);
        let len_of_encoding_bytes = 1 + ((len_of_encoding as i8 - 1) / 8) as u8;
        let mut encoding = Vec::new();
        for _i in 0..len_of_encoding_bytes {
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
        last_byte_offset,
        mappings,
        data: file,
    }
}

pub fn read_file(path: &str) -> Vec<u8> {
    fs::read(path).unwrap()
}

pub fn write_comp_file(path: &str, file: &mut File) {
    let mut chars = Vec::new();
    chars.push(file.mappings.len() as u8);
    chars.push(file.last_byte_offset);
    for mapping in &mut file.mappings {
        chars.append(&mut mapping.linearize());
    }
    chars.append(&mut file.data);
    fs::write(path, chars).unwrap();
}

pub fn write_file(path: &str, chars: Vec<u8>) {
    fs::write(path, chars).unwrap();
}
