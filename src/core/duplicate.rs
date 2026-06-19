//! Duplicate detection based on stable file fingerprints.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::{Result, VaultError};

const FNV64_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV64_PRIME: u64 = 0x0000_0100_0000_01b3;

/// Stable hash information for a file or byte slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileFingerprint {
    /// Name of the hashing algorithm.
    pub algorithm: &'static str,
    /// Hex-encoded fingerprint value.
    pub hash: String,
    /// Size of the content in bytes.
    pub byte_len: u64,
}

impl FileFingerprint {
    /// Creates a new fingerprint instance.
    pub fn new(hash: String, byte_len: u64) -> Self {
        Self {
            algorithm: "fnv1a64",
            hash,
            byte_len,
        }
    }
}

/// Computes a stable fingerprint for the provided bytes.
pub fn compute_fingerprint(bytes: &[u8]) -> FileFingerprint {
    let mut state = FNV64_OFFSET_BASIS;

    for byte in bytes {
        state ^= u64::from(*byte);
        state = state.wrapping_mul(FNV64_PRIME);
    }

    FileFingerprint::new(format!("{state:016x}"), bytes.len() as u64)
}

/// Computes a fingerprint for a file on disk.
pub fn compute_fingerprint_for_file(path: impl AsRef<Path>) -> Result<FileFingerprint> {
    let path = path.as_ref();
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(VaultError::from)?;
    Ok(compute_fingerprint(&buffer))
}

/// Returns `true` when two fingerprints describe identical content.
pub fn is_same_file(a: &FileFingerprint, b: &FileFingerprint) -> bool {
    a.hash == b.hash && a.byte_len == b.byte_len && a.algorithm == b.algorithm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprints_are_stable_for_same_bytes() {
        let first = compute_fingerprint(b"duplicate");
        let second = compute_fingerprint(b"duplicate");
        assert_eq!(first, second);
    }

    #[test]
    fn fingerprints_differ_for_different_bytes() {
        let first = compute_fingerprint(b"one");
        let second = compute_fingerprint(b"two");
        assert!(!is_same_file(&first, &second));
    }
}
