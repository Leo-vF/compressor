use std::hash::*;

pub struct CompressionHasher {
    state: u64,
}

impl Hasher for CompressionHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        self.state = *bytes.get(0).unwrap() as u64;
    }
}

pub struct BuildCompressionHasher;

impl BuildHasher for BuildCompressionHasher {
    type Hasher = CompressionHasher;
    fn build_hasher(&self) -> Self::Hasher {
        CompressionHasher { state: 0 }
    }
}

pub struct DecompressionHasher {
    state: u64,
}

impl Hasher for DecompressionHasher {
    // todo make it go faster
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state <<= 1;
            self.state += byte as u64;
        }
    }
}

pub struct BuildDecompressionHasher;

impl BuildHasher for BuildDecompressionHasher {
    type Hasher = DecompressionHasher;
    fn build_hasher(&self) -> Self::Hasher {
        DecompressionHasher { state: 1 }
    }
}
