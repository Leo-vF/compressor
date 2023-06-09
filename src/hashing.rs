use std::hash::*;

pub struct CompressionHasher {
    state: u64,
}

impl Hasher for CompressionHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        match bytes.get(0) {
            None => {}
            Some(byte) => {
                self.state = *byte as u64;
            }
        }
    }
}

pub struct BuildCompressionHasher;

impl BuildHasher for BuildCompressionHasher {
    type Hasher = CompressionHasher;
    fn build_hasher(&self) -> Self::Hasher {
        CompressionHasher { state: 0 }
    }
}
