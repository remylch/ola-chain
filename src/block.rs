use crate::hash::Hash;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub(crate) struct Block {
    id: Uuid,
    timestamp: DateTime<Utc>,
    pub(crate) previous_block_hash: Option<Hash>,
    pub(crate) current_block_hash: Hash,
    pub(crate) data: Vec<u8>,
}

impl Block {
    pub(crate) fn genesis() -> Self {
        Block {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            previous_block_hash: None,
            current_block_hash: Hash::genesis(),
            data: Vec::new(),
        }
    }

    pub(crate) fn new(data: Vec<u8>, previous_block_hash: Hash) -> Self {
        let id = Uuid::new_v4();
        let timestamp = Utc::now();

        let computed_hash = Self::compute_hash(&id, &timestamp, &previous_block_hash, &data);

        Block {
            id,
            timestamp,
            previous_block_hash: Some(previous_block_hash),
            current_block_hash: computed_hash,
            data,
        }
    }

    fn compute_hash(
        id: &Uuid,
        timestamp: &DateTime<Utc>,
        previous_hash: &Hash,
        data: &[u8],
    ) -> Hash {
        let mut hash_input = Vec::new();
        hash_input.extend_from_slice(id.as_bytes());
        hash_input.extend_from_slice(&timestamp.timestamp().to_le_bytes());
        hash_input.extend_from_slice(previous_hash.value.as_bytes());
        hash_input.extend_from_slice(data);
        Hash::new(&hash_input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis();
        assert!(!genesis.id.is_nil());
        assert!(genesis.previous_block_hash.is_none());
        assert_eq!(genesis.current_block_hash.value, "0".repeat(64));
        assert!(genesis.data.is_empty());
    }

    #[test]
    fn test_new_block() {
        let previous_hash = Hash::genesis();
        let data = b"Test block data".to_vec();
        let block = Block::new(data.clone(), previous_hash.clone());

        assert!(!block.id.is_nil());
        assert_eq!(block.previous_block_hash.unwrap().value, previous_hash.value);
        assert_eq!(block.data, data);
        assert!(!block.current_block_hash.value.is_empty());
        assert!(block.timestamp.timestamp() > 0);
    }
}
