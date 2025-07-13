use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Address {
    pub value: String,
    #[serde(skip)]
    pub raw_bytes: Option<Vec<u8>>,
}

impl Address {
    pub fn generate() -> (Self, SecretKey, PublicKey) {
        let secp = Secp256k1::new();

        let (secret_key, public_key) = secp.generate_keypair(&mut secp256k1::rand::rng());
        let pub_key_bytes = public_key.serialize_uncompressed();

        let address = Self::from_public_key(&pub_key_bytes);

        (address, secret_key, public_key)
    }

    pub fn from_public_key(pub_key: &[u8]) -> Self {
        let pub_key_bytes = if pub_key.len() == 65 && pub_key[0] == 0x04 {
            &pub_key[1..]
        } else {
            pub_key
        };

        let hash = Self::keccak256(pub_key_bytes);

        let address_bytes = &hash[12..];
        let address_str = format!("0x{}", hex::encode(address_bytes));

        Self {
            value: address_str,
            raw_bytes: Some(address_bytes.to_vec()),
        }
    }

    pub fn is_valid(&self) -> bool {
        if !self.value.starts_with("0x") || self.value.len() != 42 {
            return false;
        }

        // Check if all characters after 0x are valid hex
        self.value[2..].chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Get the raw address bytes (without 0x prefix)
    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        if self.is_valid() {
            hex::decode(&self.value[2..]).ok()
        } else {
            None
        }
    }

    fn keccak256(data: &[u8]) -> Vec<u8> {
        use sha3::{Digest, Keccak256};
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        match (&self.raw_bytes, &other.raw_bytes) {
            (Some(self_bytes), Some(other_bytes)) => self_bytes == other_bytes,
            _ => self.value == other.value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_generation() {
        let (address, secret_key, public_key) = Address::generate();

        // Address should be valid
        assert!(address.is_valid());

        // Address should start with 0x and be 42 characters
        assert!(address.value.starts_with("0x"));
        assert_eq!(address.value.len(), 42);

        // Should have raw bytes
        assert!(address.raw_bytes.is_some());
        assert_eq!(address.raw_bytes.as_ref().unwrap().len(), 20);

        // Keys should be valid
        assert_eq!(secret_key.as_ref().len(), 32);
        assert_eq!(public_key.serialize_uncompressed().len(), 65);
    }

    #[test]
    fn test_from_public_key() {
        // Test with uncompressed public key (65 bytes, starts with 0x04)
        let mock_pubkey = vec![0x04; 65];
        let address = Address::from_public_key(&mock_pubkey);

        assert!(address.is_valid());
        assert!(address.value.starts_with("0x"));
        assert_eq!(address.value.len(), 42);
        assert!(address.raw_bytes.is_some());
    }

    #[test]
    fn test_from_public_key_without_prefix() {
        // Test with 64-byte public key (without 0x04 prefix)
        let mock_pubkey = vec![0x01; 64];
        let address = Address::from_public_key(&mock_pubkey);

        assert!(address.is_valid());
        assert!(address.value.starts_with("0x"));
        assert_eq!(address.value.len(), 42);
    }

    #[test]
    fn test_address_validation() {
        // Valid address
        let valid_addr = Address {
            value: "0x742d35Cc6634C0532925a3b8D54C4d63b8Ad6a94".to_string(),
            raw_bytes: None,
        };
        assert!(valid_addr.is_valid());

        // Invalid: too short
        let short_addr = Address {
            value: "0x742d35Cc".to_string(),
            raw_bytes: None,
        };
        assert!(!short_addr.is_valid());

        // Invalid: too long
        let long_addr = Address {
            value: "0x742d35Cc6634C0532925a3b8D54C4d63b8Ad6a94abc".to_string(),
            raw_bytes: None,
        };
        assert!(!long_addr.is_valid());

        // Invalid: no 0x prefix
        let no_prefix = Address {
            value: "742d35Cc6634C0532925a3b8D54C4d63b8Ad6a94".to_string(),
            raw_bytes: None,
        };
        assert!(!no_prefix.is_valid());

        // Invalid: non-hex characters
        let invalid_chars = Address {
            value: "0x742d35Cc6634C0532925a3b8D54C4d63b8Ad6aZZ".to_string(),
            raw_bytes: None,
        };
        assert!(!invalid_chars.is_valid());
    }

    #[test]
    fn test_as_bytes() {
        let valid_addr = Address {
            value: "0x742d35Cc6634C0532925a3b8D54C4d63b8Ad6a94".to_string(),
            raw_bytes: None,
        };

        let bytes = valid_addr.as_bytes();
        assert!(bytes.is_some());
        assert_eq!(bytes.unwrap().len(), 20);

        // Invalid address should return None
        let invalid_addr = Address {
            value: "invalid".to_string(),
            raw_bytes: None,
        };
        assert!(invalid_addr.as_bytes().is_none());
    }

    #[test]
    fn test_address_equality() {
        let addr1 = Address {
            value: "0x742d35Cc6634C0532925a3b8D54C4d63b8Ad6a94".to_string(),
            raw_bytes: Some(vec![1, 2, 3]),
        };

        let addr2 = Address {
            value: "0x742d35Cc6634C0532925a3b8D54C4d63b8Ad6a94".to_string(),
            raw_bytes: Some(vec![1, 2, 3]),
        };

        let addr3 = Address {
            value: "0x742d35Cc6634C0532925a3b8D54C4d63b8Ad6a94".to_string(),
            raw_bytes: Some(vec![4, 5, 6]),
        };

        let addr4 = Address {
            value: "0x742d35Cc6634C0532925a3b8D54C4d63b8Ad6a94".to_string(),
            raw_bytes: None,
        };

        // Same raw bytes should be equal
        assert_eq!(addr1, addr2);

        // Different raw bytes should not be equal
        assert_ne!(addr1, addr3);

        // Falls back to string comparison when raw_bytes is None
        assert_eq!(addr1, addr4);
    }

    #[test]
    fn test_keccak256_deterministic() {
        let data = b"test data";
        let hash1 = Address::keccak256(data);
        let hash2 = Address::keccak256(data);

        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 32); // Keccak256 produces 32 bytes
    }

    #[test]
    fn test_generation_produces_unique_addresses() {
        let (addr1, _, _) = Address::generate();
        let (addr2, _, _) = Address::generate();

        // Should be extremely unlikely to generate the same address twice
        assert_ne!(addr1.value, addr2.value);
    }

    #[test]
    fn test_ethereum_compatibility() {
        // Test that the address format matches Ethereum standards
        let (address, _, _) = Address::generate();

        // Should be lowercase hex after 0x
        let hex_part = &address.value[2..];
        assert!(hex_part.chars().all(|c| c.is_ascii_hexdigit() && (c.is_ascii_lowercase() || c.is_ascii_digit())));

        // Should be exactly 40 hex characters (20 bytes)
        assert_eq!(hex_part.len(), 40);
    }
}