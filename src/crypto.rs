use sha2::{Digest, Sha256};

pub fn double_hash(data: &[u8]) -> [u8; 32] {
    Sha256::digest(Sha256::digest(data)).into()
}

pub fn to_hash(unhashed: &str) -> [u8; 32] {
    Sha256::digest(unhashed.as_bytes()).into()
}
