//! Minimal primitive types for consensus-related structures.
//! These types are defined independently to avoid external dependencies.

use serde::{Deserialize, Serialize};

/// Digest length in bytes (32 bytes for SHA-256)
pub const DIGEST_LENGTH: usize = 32;

/// Authority index type (typically u16 or u32)
pub type AuthorityIndex = u32;

/// Block reference - a unique identifier for a block
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct BlockRef {
    /// The address of the leader in format 0x{20-bytes hex string}
    pub leader_address: String,
    /// The digest of the block
    pub digest: [u8; DIGEST_LENGTH],
    /// The round number
    pub round: u64,
}

/// Commit reference - a unique identifier for a commit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct CommitRef {
    /// The digest of the commit
    pub digest: [u8; DIGEST_LENGTH],
    /// The round number
    pub round: usize,
}

/// Block timestamp in milliseconds
pub type BlockTimestampMs = u64;

/// Transaction type - a simple wrapper around raw bytes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    inner: Vec<u8>,
}

impl Transaction {
    /// Create a new transaction from raw bytes
    pub fn new(data: Vec<u8>) -> Self {
        Self { inner: data }
    }

    /// Get the transaction data
    pub fn data(&self) -> &[u8] {
        &self.inner
    }

    /// Consume the transaction and return the inner data
    pub fn into_data(self) -> Vec<u8> {
        self.inner
    }
}

impl AsRef<[u8]> for Transaction {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_ref_default() {
        let block_ref = BlockRef::default();
        assert_eq!(block_ref.digest, [0u8; DIGEST_LENGTH]);
        assert_eq!(block_ref.round, 0);
    }

    #[test]
    fn test_block_ref_equality() {
        let mut digest1 = [0u8; DIGEST_LENGTH];
        digest1[0] = 1;
        let block_ref1 = BlockRef {
            digest: digest1,
            round: 10,
            leader_address: String::new(),
            ..Default::default()
        };
        let block_ref2 = BlockRef {
            digest: digest1,
            round: 10,
            leader_address: String::new(),
            ..Default::default()
        };
        let block_ref3 = BlockRef {
            digest: digest1,
            round: 11,
            leader_address: String::new(),
            ..Default::default()
        };
        assert_eq!(block_ref1, block_ref2);
        assert_ne!(block_ref1, block_ref3);
    }

    #[test]
    fn test_block_ref_serialization() {
        let mut digest = [0u8; DIGEST_LENGTH];
        digest[0] = 42;
        let block_ref = BlockRef {
            digest,
            round: 100,
            leader_address: String::new(),
            ..Default::default()
        };
        let serialized = serde_json::to_string(&block_ref).unwrap();
        let deserialized: BlockRef = serde_json::from_str(&serialized).unwrap();
        assert_eq!(block_ref, deserialized);
    }

    #[test]
    fn test_commit_ref_default() {
        let commit_ref = CommitRef::default();
        assert_eq!(commit_ref.digest, [0u8; DIGEST_LENGTH]);
        assert_eq!(commit_ref.round, 0);
    }

    #[test]
    fn test_commit_ref_equality() {
        let mut digest1 = [0u8; DIGEST_LENGTH];
        digest1[0] = 2;
        let commit_ref1 = CommitRef {
            digest: digest1,
            round: 20,
        };
        let commit_ref2 = CommitRef {
            digest: digest1,
            round: 20,
        };
        let commit_ref3 = CommitRef {
            digest: digest1,
            round: 21,
        };
        assert_eq!(commit_ref1, commit_ref2);
        assert_ne!(commit_ref1, commit_ref3);
    }

    #[test]
    fn test_commit_ref_serialization() {
        let mut digest = [0u8; DIGEST_LENGTH];
        digest[0] = 99;
        let commit_ref = CommitRef { digest, round: 200 };
        let serialized = serde_json::to_string(&commit_ref).unwrap();
        let deserialized: CommitRef = serde_json::from_str(&serialized).unwrap();
        assert_eq!(commit_ref, deserialized);
    }

    #[test]
    fn test_transaction_new() {
        let data = vec![1, 2, 3, 4, 5];
        let tx = Transaction::new(data.clone());
        assert_eq!(tx.data(), &data);
    }

    #[test]
    fn test_transaction_data() {
        let data = vec![10, 20, 30];
        let tx = Transaction::new(data.clone());
        assert_eq!(tx.data(), &data);
    }

    #[test]
    fn test_transaction_into_data() {
        let data = vec![100, 200, 255];
        let tx = Transaction::new(data.clone());
        let extracted = tx.into_data();
        assert_eq!(extracted, data);
    }

    #[test]
    fn test_transaction_as_ref() {
        let data = vec![5, 10, 15, 20];
        let tx = Transaction::new(data.clone());
        assert_eq!(tx.as_ref(), &data);
    }

    #[test]
    fn test_transaction_equality() {
        let data1 = vec![1, 2, 3];
        let data2 = vec![1, 2, 3];
        let data3 = vec![1, 2, 4];
        let tx1 = Transaction::new(data1);
        let tx2 = Transaction::new(data2);
        let tx3 = Transaction::new(data3);
        assert_eq!(tx1, tx2);
        assert_ne!(tx1, tx3);
    }

    #[test]
    fn test_transaction_serialization() {
        let data = vec![1, 2, 3, 4, 5];
        let tx = Transaction::new(data.clone());
        let serialized = serde_json::to_string(&tx).unwrap();
        let deserialized: Transaction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(tx.data(), deserialized.data());
    }

    #[test]
    fn test_transaction_clone() {
        let data = vec![42, 43, 44];
        let tx1 = Transaction::new(data);
        let tx2 = tx1.clone();
        assert_eq!(tx1.data(), tx2.data());
    }
}
