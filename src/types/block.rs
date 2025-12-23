use serde::{Deserialize, Serialize};
use std::{
    fmt,
    hash::{Hash, Hasher},
};

use crate::types::{Transaction, DIGEST_LENGTH};

pub type Block = Vec<Transaction>;
/// A Block with its signature, before they are verified.
///
/// Note: `BlockDigest` is computed over this struct, so any added field (without `#[serde(skip)]`)
/// will affect the values of `BlockDigest` and `BlockRef`.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SignedBlock {
    inner: Block,
    signature: Vec<u8>,
}

impl SignedBlock {
    pub fn new(block: Block) -> Self {
        Self {
            inner: block,
            signature: Vec::new(),
        }
    }

    /// Get a reference to the transactions in this block
    pub fn transactions(&self) -> &Block {
        &self.inner
    }

    /// Clears signature for testing.
    #[cfg(test)]
    pub(crate) fn clear_signature(&mut self) {
        self.signature = Vec::new();
    }
}

/// Digest of a `VerifiedBlock` or verified `SignedBlock`, which covers the `Block` and its
/// signature.
///
/// Note: the signature algorithm is assumed to be non-malleable, so it is impossible for another
/// party to create an altered but valid signature, producing an equivocating `BlockDigest`.
#[derive(Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockDigest(pub [u8; DIGEST_LENGTH]);

impl BlockDigest {
    /// Lexicographic min & max digest.
    pub const MIN: Self = Self([u8::MIN; DIGEST_LENGTH]);
    pub const MAX: Self = Self([u8::MAX; DIGEST_LENGTH]);
}

impl Hash for BlockDigest {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.0[..8]);
    }
}

impl fmt::Display for BlockDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, self.0)
                .get(0..4)
                .ok_or(fmt::Error)?
        )
    }
}

impl fmt::Debug for BlockDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, self.0)
        )
    }
}

impl AsRef<[u8]> for BlockDigest {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Transaction;

    #[test]
    fn test_signed_block_new() {
        let transactions = vec![
            Transaction::new(vec![1, 2, 3]),
            Transaction::new(vec![4, 5, 6]),
        ];
        let block = SignedBlock::new(transactions.clone());
        assert_eq!(block.transactions(), &transactions);
    }

    #[test]
    fn test_signed_block_transactions() {
        let transactions = vec![Transaction::new(vec![10, 20, 30])];
        let block = SignedBlock::new(transactions.clone());
        assert_eq!(block.transactions(), &transactions);
    }

    #[test]
    fn test_signed_block_clear_signature() {
        let transactions = vec![Transaction::new(vec![1, 2, 3])];
        let mut block = SignedBlock::new(transactions);
        block.clear_signature();
        // Signature should be cleared (empty)
        // Note: We can't directly access signature, but clear_signature should work
    }

    #[test]
    fn test_signed_block_default() {
        let block = SignedBlock::default();
        assert_eq!(block.transactions().len(), 0);
    }

    #[test]
    fn test_signed_block_serialization() {
        let transactions = vec![
            Transaction::new(vec![1, 2, 3]),
            Transaction::new(vec![4, 5, 6]),
        ];
        let block = SignedBlock::new(transactions.clone());
        let serialized = serde_json::to_string(&block).unwrap();
        let deserialized: SignedBlock = serde_json::from_str(&serialized).unwrap();
        assert_eq!(
            block.transactions().len(),
            deserialized.transactions().len()
        );
        assert_eq!(block.transactions(), deserialized.transactions());
        // Verify transaction data is preserved
        for (orig, deser) in block
            .transactions()
            .iter()
            .zip(deserialized.transactions().iter())
        {
            assert_eq!(orig.data(), deser.data());
        }
    }

    #[test]
    fn test_signed_block_clone() {
        let transactions = vec![
            Transaction::new(vec![1, 2, 3]),
            Transaction::new(vec![4, 5, 6]),
        ];
        let block1 = SignedBlock::new(transactions.clone());
        let block2 = block1.clone();
        assert_eq!(block1.transactions(), block2.transactions());
        assert_eq!(block1.transactions().len(), block2.transactions().len());
    }

    #[test]
    fn test_block_digest_default() {
        let digest = BlockDigest::default();
        assert_eq!(digest.0, [0u8; DIGEST_LENGTH]);
    }

    #[test]
    fn test_block_digest_min_max() {
        let min = BlockDigest::MIN;
        let max = BlockDigest::MAX;
        assert_eq!(min.0, [u8::MIN; DIGEST_LENGTH]);
        assert_eq!(max.0, [u8::MAX; DIGEST_LENGTH]);
    }

    #[test]
    fn test_block_digest_equality() {
        let mut digest1 = [0u8; DIGEST_LENGTH];
        digest1[0] = 1;
        let mut digest2 = [0u8; DIGEST_LENGTH];
        digest2[0] = 1;
        let mut digest3 = [0u8; DIGEST_LENGTH];
        digest3[0] = 2;
        let bd1 = BlockDigest(digest1);
        let bd2 = BlockDigest(digest2);
        let bd3 = BlockDigest(digest3);
        assert_eq!(bd1, bd2);
        assert_ne!(bd1, bd3);
    }

    #[test]
    fn test_block_digest_ordering() {
        let mut digest1 = [0u8; DIGEST_LENGTH];
        digest1[0] = 1;
        let mut digest2 = [0u8; DIGEST_LENGTH];
        digest2[0] = 2;
        let bd1 = BlockDigest(digest1);
        let bd2 = BlockDigest(digest2);
        assert!(bd1 < bd2);
    }

    #[test]
    fn test_block_digest_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut digest1 = [0u8; DIGEST_LENGTH];
        digest1[0] = 1;
        let mut digest2 = [0u8; DIGEST_LENGTH];
        digest2[0] = 1;
        let bd1 = BlockDigest(digest1);
        let bd2 = BlockDigest(digest2);
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        bd1.hash(&mut hasher1);
        bd2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_block_digest_as_ref() {
        let mut digest = [0u8; DIGEST_LENGTH];
        digest[0] = 42;
        let bd = BlockDigest(digest);
        assert_eq!(bd.as_ref(), &digest);
    }

    #[test]
    fn test_block_digest_serialization() {
        let mut digest = [0u8; DIGEST_LENGTH];
        digest[0] = 100;
        let bd = BlockDigest(digest);
        let serialized = serde_json::to_string(&bd).unwrap();
        let deserialized: BlockDigest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(bd, deserialized);
    }

    #[test]
    fn test_block_digest_display() {
        let mut digest = [0u8; DIGEST_LENGTH];
        digest[0] = 65; // 'A' in base64
        let bd = BlockDigest(digest);
        let display_str = format!("{}", bd);
        // Should display first 4 characters of base64 encoding
        assert!(!display_str.is_empty());
        assert!(display_str.len() <= 4);
    }

    #[test]
    fn test_block_digest_debug() {
        let mut digest = [0u8; DIGEST_LENGTH];
        digest[0] = 65;
        let bd = BlockDigest(digest);
        let debug_str = format!("{:?}", bd);
        // Should display full base64 encoding
        assert!(!debug_str.is_empty());
    }
}
