use std::hash::*;

pub struct CompressionHasher {
    state: u64,
}

impl Hasher for CompressionHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.state += *byte as u64;
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
