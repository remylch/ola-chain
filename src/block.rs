use crate::hash::Hash;
use crate::transaction::Transaction;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_block_hash: Option<Hash>,
    pub current_block_hash: Option<Hash>,
    pub merkle_root: Hash,
    pub data: Vec<u8>,
    pub nonce: u64,
    pub difficulty: u32,
}

impl Block {
    pub(crate) fn genesis() -> Self {
        let mut genesis_block = Self {
            index: 0,
            timestamp: Utc::now(),
            previous_block_hash: None,
            current_block_hash: None,
            merkle_root: Hash::genesis(),
            data: Vec::new(),
            nonce: 0,
            transactions: Vec::new(),
            difficulty: 4,
        };

        genesis_block.current_block_hash = Some(genesis_block.compute_hash());
        genesis_block
    }

    pub(crate) fn new(index: u64, transactions: Vec<Transaction>, previous_block_hash: Hash) -> Self {
        let mut new_block = Self {
            index,
            timestamp: Utc::now(),
            transactions: transactions.clone(),
            previous_block_hash: Some(previous_block_hash),
            current_block_hash: None, // Not computed yet
            merkle_root: Self::calculate_merkle_root(&transactions),
            data: Vec::new(),
            nonce: 0,
            difficulty: 4,
        };

        // Calculate the actual hash for the new block
        new_block.current_block_hash = Some(new_block.compute_hash());
        new_block
    }

    fn calculate_merkle_root(transactions: &[Transaction]) -> Hash {
        if transactions.is_empty() {
            return Hash::new(&[]);
        }

        // Simple merkle root calculation (concatenate all transaction IDs)
        let mut merkle_input = Vec::new();
        for tx in transactions {
            merkle_input.extend_from_slice(tx.id.as_bytes());
        }

        Hash::new(&merkle_input)
    }

    fn compute_hash(&self) -> Hash {
        let mut hash_input = Vec::new();

        // Add block metadata
        hash_input.extend_from_slice(&self.index.to_le_bytes());
        hash_input.extend_from_slice(&self.timestamp.timestamp().to_le_bytes());
        hash_input.extend_from_slice(&self.nonce.to_le_bytes());
        hash_input.extend_from_slice(&self.difficulty.to_le_bytes());

        // Add previous block hash if it exists
        if let Some(prev_hash) = &self.previous_block_hash {
            hash_input.extend_from_slice(prev_hash.value.as_bytes());
        }

        // Add merkle root
        hash_input.extend_from_slice(self.merkle_root.value.as_bytes());

        // Add transaction data
        for transaction in &self.transactions {
            if let Ok(tx_bytes) = serde_json::to_vec(transaction) {
                hash_input.extend_from_slice(&tx_bytes);
            }
        }

        // Add additional data
        hash_input.extend_from_slice(&self.data);

        Hash::new(&hash_input)
    }

    pub fn mine_block(&mut self, target_difficulty: u32) {
        let target = "0".repeat(target_difficulty as usize);

        loop {
            let hash = self.compute_hash();
            if hash.value.starts_with(&target) {
                self.current_block_hash = Some(hash);
                break;
            }
            self.nonce += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::Address;

    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis();
        assert_eq!(genesis.index, 0);
        assert!(genesis.previous_block_hash.is_none());
        assert!(genesis.data.is_empty());
        assert!(genesis.transactions.is_empty());
        assert_eq!(genesis.difficulty, 4);
    }

    #[test]
    fn test_new_block() {
        let previous_hash = Hash::genesis();
        let address1 = Address::generate().0;
        let address2 = Address::generate().0;
        let transactions = vec![
            Transaction::new(
                address1,
                address2,
                100
            )
        ];

        let block = Block::new(1, transactions.clone(), previous_hash.clone());

        assert_eq!(block.index, 1);
        assert_eq!(block.previous_block_hash.unwrap().value, previous_hash.value);
        assert_eq!(block.transactions.len(), 1);
        assert_eq!(block.difficulty, 4);
        assert!(block.timestamp.timestamp() > 0);
    }

    #[test]
    fn test_block_hash_calculation() {
        let genesis = Block::genesis();
        let calculated_hash = genesis.compute_hash();

        // Hash should be deterministic
        assert_eq!(genesis.current_block_hash.unwrap().value, calculated_hash.value);
    }

    #[test]
    fn test_merkle_root_calculation() {
        let address1 = Address::generate().0;
        let address2 = Address::generate().0;
        let address3 = Address::generate().0;

        let transactions = vec![
            Transaction::new(
                address1,
                address2.clone(),
                50
            ),
            Transaction::new(
                address2,
                address3,
                25
            )
        ];

        let merkle_root = Block::calculate_merkle_root(&transactions);
        assert!(!merkle_root.value.is_empty());

        // Empty transactions should return genesis hash
        let empty_merkle = Block::calculate_merkle_root(&[]);
        assert_eq!(empty_merkle.value, Hash::new(&[]).value);
    }
}
