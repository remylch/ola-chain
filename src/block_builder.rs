use crate::block::Block;
use crate::chain::Chain;
use crate::hash::Hash;
use crate::store::StoreError;
use crate::transaction::Transaction;
use crate::transaction_pool::TransactionPool;

#[derive(Clone)]
pub struct BlockBuilder {
    transaction_pool: TransactionPool,
    current_block: Option<Block>,
    blockchain: Chain,
    block_time_limit: u64,
    min_transactions: usize,
    last_block_time: u64,
}

impl BlockBuilder {

    pub fn new(chain: Chain) -> Self {
        Self {
            transaction_pool: TransactionPool::new(1000, 1024*1024), // 1000 txs, 1MB max
            current_block: None,
            blockchain: chain,
            block_time_limit: 600, // 10 minutes
            min_transactions: 1,
            last_block_time: 0,
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        self.transaction_pool.add_transaction(transaction)
    }

    pub fn should_create_block(&self) -> bool {
        let now = chrono::Utc::now().timestamp() as u64;
        let time_elapsed = now - self.last_block_time;

        time_elapsed >= self.block_time_limit ||
            self.transaction_pool.pending_count() >= self.min_transactions
    }

    pub fn create_block(&mut self) -> Option<Block> {
        if !self.should_create_block() {
            return None;
        }

        let transactions = self.transaction_pool.pull_transactions_for_block();
        if transactions.is_empty() {
            return None;
        }

        let previous_block = self.blockchain.blocks.last()?;
        let new_index = previous_block.index + 1;
        let previous_hash = previous_block.current_block_hash.clone()?;

        let block = Block::new(new_index, transactions, previous_hash);
        self.last_block_time = chrono::Utc::now().timestamp() as u64;

        Some(block)
    }

    pub fn mine_and_add_block(&mut self) -> Result<Hash, StoreError> {
        if let Some(mut block) = self.create_block() {
            block.mine_block(block.difficulty);
            self.blockchain.add_block(block)
        } else {
            Err(StoreError::NoBlockToCreate())
        }
    }

    pub fn get_pending_transaction_count(&self) -> usize {
        self.transaction_pool.pending_count()
    }

}