use crate::block::Block;
use crate::hash::Hash;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, fs};
use crate::store::{Store, StoreError};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Chain {
    difficulty: i8,
    genesis_block_hash: Hash,
    initialized_at: DateTime<Utc>,
    #[serde(skip)]
    pub(crate) blocks: Vec<Block>,
}

impl Chain {
    pub(crate) fn load_or_create() -> Self {
        let base_path = env::var("BLOCKCHAIN_DATA_PATH").unwrap_or_else(|_| ".".to_string());
        let blockchain_file = format!("{}/blockchain.json", base_path);

        if let Some(parent) = std::path::Path::new(&blockchain_file).parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|e| {
                eprintln!("Failed to create data directory: {}", e);
            });
        }

        if Path::new(&blockchain_file).exists() {
            println!("Loading Blockchain from file...");
            Self::load_from_file(&blockchain_file)
        } else {
            println!("Initializing new Blockchain...");
            Self::create_new_chain(blockchain_file)
        }
    }

    pub(crate) fn add_block(&mut self, block: Block) -> Result<Hash, StoreError> {
        let hash = self.save(block)?;
        Ok(hash)
    }

    fn create_new_chain(file_to_save: String) -> Self {
        let initialized_at = Utc::now();
        let genesis_block = Block::genesis();
        let genesis_block_hash = genesis_block.current_block_hash.clone().unwrap();

        let chain = Chain {
            initialized_at,
            genesis_block_hash,
            difficulty: 4,
            blocks: vec![genesis_block],
        };

        chain.save_to_file(&file_to_save);
        chain
    }

    fn load_from_file(blockchain_file: &str) -> Chain {
        match fs::read_to_string(blockchain_file) {
            Ok(content) => {
                serde_json::from_str::<Chain>(&content).unwrap_or_else(|e| {
                    panic!("Failed to parse blockchain file: {}", e)
                })
            }
            Err(e) => {
                panic!("Failed to read blockchain file: {}", e);
            }
        }
    }

    fn save_to_file(&self, filename: &str) {
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if let Err(e) = fs::write(filename, json) {
                    eprintln!("Failed to save blockchain to {}: {}", filename, e);
                } else {
                    println!("Blockchain saved to {}", filename);
                }
            }
            Err(e) => {
                eprintln!("Error serializing blockchain: {}", e);
                return;
            }
        };
    }

}
