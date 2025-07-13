use std::collections::VecDeque;
use crate::transaction::Transaction;

#[derive(Clone)]
pub struct TransactionPool {
    pending_transactions: VecDeque<Transaction>,
    by_fee: std::collections::BTreeMap<u64, Vec<Transaction>>,
    max_transactions_per_block: usize,
    max_block_size: usize,
}

impl TransactionPool {
    pub fn new(max_transactions_per_block: usize, max_block_size: usize) -> Self {
        Self {
            pending_transactions: VecDeque::new(),
            by_fee: std::collections::BTreeMap::new(),
            max_transactions_per_block,
            max_block_size
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        if !transaction.is_valid() {
            return Err("Invalid transaction".to_string());
        }

        if self.pending_transactions.len() >= self.max_transactions_per_block {
            return Err("Transaction pool is full".to_string());
        }

        let fee = transaction.fee;
        self.pending_transactions.push_back(transaction.clone());
        self.by_fee.entry(fee).or_insert_with(Vec::new).push(transaction);
        Ok(())
    }

    pub fn pull_transactions_for_block(&mut self) -> Vec<Transaction> {
        let mut selected_txs = Vec::new();
        let mut total_size = 0;
        let mut tx_id_to_remove = Vec::new();

        for (_fee, transactions) in self.by_fee.iter().rev() {
            for tx in transactions {
                let tx_size = self.estimate_transaction_size(tx);

                if selected_txs.len() >= self.max_transactions_per_block || total_size + tx_size > self.max_block_size {
                    break;
                }

                selected_txs.push(tx.clone());
                tx_id_to_remove.push(tx.id.clone());
                total_size += tx_size;

                if selected_txs.len() >= self.max_transactions_per_block {
                    break;
                }
            }

            if selected_txs.len() >= self.max_transactions_per_block {
                break;
            }

        }

        for tx_id in tx_id_to_remove {
            self.remove_transaction(&tx_id);
        }

        selected_txs
    }

    pub fn estimate_transaction_size(&self, transaction: &Transaction) -> usize {
        serde_json::to_string(transaction).unwrap_or_default().len()
    }

    pub fn remove_transaction(&mut self, transaction_id: &str) {
        self.pending_transactions.retain(|tx| tx.id != transaction_id);
        for (_, transactions) in self.by_fee.iter_mut() {
            transactions.retain(|tx| tx.id != transaction_id);
        }
        self.by_fee.retain(|_, tx| !tx.is_empty());
    }

    pub fn pending_count(&self) -> usize {
        self.pending_transactions.len()
    }

}