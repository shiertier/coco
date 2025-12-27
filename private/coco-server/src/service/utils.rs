use sha2::{Digest, Sha256};

pub(crate) fn generate_id(prefix: &str, seed: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    let digest = hasher.finalize();
    format!("{prefix}-{}", bytes_to_hex(&digest))
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(hex_digit(byte >> 4));
        output.push(hex_digit(byte & 0x0f));
    }
    output
}

fn hex_digit(value: u8) -> char {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    HEX[value as usize] as char
}
