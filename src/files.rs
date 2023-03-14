use std::fs;

#[derive(Debug)]
pub struct Count {
    pub char: u8,
    pub count: u32,
}

#[derive(Debug)]
pub struct File {
    pub probs: Vec<Count>,
    pub data: Vec<u8>,
}

pub fn read_comp_file(path: &str) -> File {
    let mut file: Vec<u8> = fs::read(path).unwrap();
    let n_counts = file.remove(0) as usize;
    let mut counts = Vec::new();
    let mut n = 0;

    while n >= 5 * n_counts {
        counts.push(Count {
            char: file[n],
            count: u32::from_be_bytes([file[n + 1], file[n + 2], file[n + 3], file[n + 4]]),
        });
        n += 5;
    }
    File {
        probs: counts,
        data: vec![file[5 * n_counts]],
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
