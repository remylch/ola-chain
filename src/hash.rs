use serde::{Deserialize, Serialize};
use sha2::digest::Update;
use sha2::{Digest, Sha256};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Hash {
    pub(crate) value: String,
}

impl Hash {
    pub(crate) fn new(value: String) -> Self {
        Hash { value }
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
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
