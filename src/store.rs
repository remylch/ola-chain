use crate::block::Block;
use crate::chain::Chain;
use crate::hash::Hash;
use std::fmt;
use std::fmt::Formatter;

pub trait Store<T> {
    fn save(&mut self, item: T) -> Result<Hash, StoreError>;
}

#[derive(Debug)]
pub enum StoreError {
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
    ValidationError(String),
    NoBlockToCreate(),
    DuplicateBlockError(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StoreError::IoError(e) => write!(f, "IO error {}", e),
            StoreError::SerializationError(e) => write!(f, "Serialization error {}", e),
            StoreError::ValidationError(e) => write!(f, "Validation error: {}", e),
            StoreError::DuplicateBlockError(e) => write!(f, "Duplicate block error: {}", e),
            StoreError::NoBlockToCreate() => write!(f, "No block to create in transaction pool"),
        }
    }
}

impl Store<Block> for Chain {
    fn save(&mut self, block: Block) -> Result<Hash, StoreError> {
        let hash = block.current_block_hash.clone().unwrap();
        self.blocks.push(block);
        //TODO: Write it to disk ?
        Ok(hash)
    }
}