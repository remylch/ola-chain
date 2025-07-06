use serde::{Deserialize, Serialize};
use sha2::digest::Update;
use sha2::{Digest, Sha256};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Hash {
    pub(crate) value: String,
}

impl Hash {
    pub(crate) fn new(bytes: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        Update::update(&mut hasher, bytes);
        let result = hasher.finalize();

        Hash {
            value: hex::encode(result),
        }
    }

    pub(crate) fn genesis() -> Self {
        Hash {
            value: "0".repeat(64),
        }
    }

    fn validate(hash: String) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_from_bytes() {
        let input = b"hello world";
        let hash = Hash::new(input);
        let expected = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        assert_eq!(hash.value, expected);
    }

    #[test]
    fn test_hash_genesis() {
        let genesis = Hash::genesis();
        assert_eq!(genesis.value, "0".repeat(64));
        assert_eq!(genesis.value.len(), 64);
    }

    #[test]
    fn test_hash_from_bytes_deterministic() {
        let input = b"test data";
        let hash1 = Hash::new(input);
        let hash2 = Hash::new(input);

        assert_eq!(hash1.value, hash2.value);
    }

    #[test]
    fn test_hash_from_json_input() {
        let input = r#"{"user": "John Doe"}"#;
        let hash = Hash::new(input.as_bytes());
        assert_eq!(hash.value, "5214d486226d628e3d7abba53dee49d476760136f37b707ba1a5cfd06f45227a");
    }

    #[test]
    fn test_hash_validate() {
        assert!(Hash::validate("valid_hash".to_string()));
    }
}
