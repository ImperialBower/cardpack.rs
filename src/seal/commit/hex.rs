//! Tiny hex codec. Private: `Commitment`/`CombinedSeed` print and parse hex,
//! nothing else does, and a `hex` dependency for two functions is not worth a
//! ban-list row.

use alloc::string::String;

const DIGITS: &[u8; 16] = b"0123456789abcdef";

pub fn encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push(DIGITS[usize::from(b >> 4)] as char);
        s.push(DIGITS[usize::from(b & 0x0f)] as char);
    }
    s
}

/// Exactly 64 hex digits (either case) → 32 bytes. `None` otherwise.
pub fn decode_32(s: &str) -> Option<[u8; 32]> {
    let src = s.as_bytes();
    if src.len() != 64 {
        return None;
    }
    let mut out = [0u8; 32];
    for (i, pair) in src.chunks_exact(2).enumerate() {
        let hi = nibble(pair[0])?;
        let lo = nibble(pair[1])?;
        out[i] = (hi << 4) | lo;
    }
    Some(out)
}

fn nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}
