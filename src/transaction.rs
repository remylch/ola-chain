use crate::address::Address;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub trait Signer {
    fn sign(&self, transaction: &Transaction) -> String;
    fn verify_signature(&self, transaction: &Transaction) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Transaction {
    pub id: String,
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub fee: u64,
    pub timestamp: u64,
    pub signature: Option<String>,
}

impl Transaction {
    pub fn new(from: Address, to: Address, amount: u64) -> Self {
        let mut tx = Self {
            id: String::new(),
            fee: 0,
            from,
            to,
            amount,
            timestamp: chrono::Utc::now().timestamp() as u64,
            signature: None,
        };

        let hash = tx.calculate_hash();
        tx.id = hex::encode(hash);
        tx
    }

    pub fn sign(&mut self, private_key: &SecretKey) -> Result<(), String> {
        let secp = Secp256k1::new();

        let tx_hash = self.calculate_hash();
        let message = secp256k1::Message::from_digest(tx_hash);

        let signature = secp.sign_ecdsa(message, private_key);
        self.signature = Some(hex::encode(signature.serialize_compact()));

        Ok(())
    }

    pub fn verify_signature(&self, public_key: PublicKey) -> bool {
        let Some(ref sig_str) = self.signature else {
            return false;
        };

        let secp = Secp256k1::new();

        let Ok(sig_bytes) = hex::decode(sig_str) else {
            return false;
        };

        let Ok(signature) = secp256k1::ecdsa::Signature::from_compact(&sig_bytes) else {
            return false;
        };

        let tx_hash = self.calculate_hash();
        let message = secp256k1::Message::from_digest(tx_hash);

        secp.verify_ecdsa(message, &signature, &public_key).is_ok()
    }

    pub fn is_valid(&self) -> bool {
        self.amount > 0 && self.from != self.to && self.signature.is_some()
    }

    fn calculate_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        // Add transaction fields to hash input
        hasher.update(self.from.value.as_bytes());
        hasher.update(self.to.value.as_bytes());
        hasher.update(&self.amount.to_le_bytes());
        hasher.update(&self.fee.to_le_bytes());
        hasher.update(&self.timestamp.to_le_bytes());

        let result = hasher.finalize();
        result.into()
    }
}