use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::types::{AuthorityIndex, BlockRef, BlockTimestampMs, CommitRef};
use crate::{BlockDigest, SignedBlock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedBlock {
    pub block: SignedBlock,
    pub digest: BlockDigest,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommittedSubDag {
    pub leader: BlockRef,
    pub blocks: Vec<VerifiedBlock>,
    pub timestamp_ms: BlockTimestampMs,
    pub commit_ref: CommitRef,
    pub reputation_scores_desc: Vec<(AuthorityIndex, u64)>,
}
impl CommittedSubDag {
    pub fn flatten_transactions(&self) -> Vec<Vec<u8>> {
        self.blocks
            .iter()
            .flat_map(|block| {
                block
                    .block
                    .transactions()
                    .iter()
                    .map(|tx| tx.data().to_vec())
            })
            .collect()
    }
    pub fn len(&self) -> usize {
        self.blocks
            .iter()
            .map(|block| block.block.transactions().len())
            .sum()
    }
}
// Note: If you need to convert from external consensus types, implement From trait
// for your specific consensus library types. This keeps the crate independent.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{BlockRef, CommitRef, Transaction};
    use crate::{BlockDigest, SignedBlock};

    fn create_test_block_ref(round: u64) -> BlockRef {
        let mut digest = [0u8; 32];
        digest[0] = round as u8;
        BlockRef { digest, round }
    }

    fn create_test_commit_ref(round: usize) -> CommitRef {
        let mut digest = [0u8; 32];
        digest[0] = round as u8;
        CommitRef { digest, round }
    }

    fn create_test_signed_block(transactions: Vec<Transaction>) -> SignedBlock {
        SignedBlock::new(transactions)
    }

    fn create_test_verified_block(transactions: Vec<Transaction>) -> VerifiedBlock {
        let block = create_test_signed_block(transactions);
        let mut digest = [0u8; 32];
        digest[0] = 1;
        VerifiedBlock {
            block,
            digest: BlockDigest(digest),
        }
    }

    #[test]
    fn test_committed_subdag_default() {
        let subdag = CommittedSubDag::default();
        assert_eq!(subdag.blocks.len(), 0);
        assert_eq!(subdag.timestamp_ms, 0);
        assert_eq!(subdag.reputation_scores_desc.len(), 0);
    }

    #[test]
    fn test_committed_subdag_flatten_transactions() {
        let block1 = create_test_verified_block(vec![
            Transaction::new(vec![1, 2, 3]),
            Transaction::new(vec![4, 5, 6]),
        ]);
        let block2 = create_test_verified_block(vec![Transaction::new(vec![7, 8, 9])]);
        let subdag = CommittedSubDag {
            leader: create_test_block_ref(1),
            blocks: vec![block1, block2],
            timestamp_ms: 1000,
            commit_ref: create_test_commit_ref(1),
            reputation_scores_desc: vec![],
        };
        let flattened = subdag.flatten_transactions();
        assert_eq!(flattened.len(), 3);
        assert_eq!(flattened[0], vec![1, 2, 3]);
        assert_eq!(flattened[1], vec![4, 5, 6]);
        assert_eq!(flattened[2], vec![7, 8, 9]);
    }

    #[test]
    fn test_committed_subdag_len() {
        let block1 = create_test_verified_block(vec![
            Transaction::new(vec![1, 2, 3]),
            Transaction::new(vec![4, 5, 6]),
        ]);
        let block2 = create_test_verified_block(vec![Transaction::new(vec![7, 8, 9])]);
        let subdag = CommittedSubDag {
            leader: create_test_block_ref(1),
            blocks: vec![block1, block2],
            timestamp_ms: 1000,
            commit_ref: create_test_commit_ref(1),
            reputation_scores_desc: vec![],
        };
        assert_eq!(subdag.len(), 3);
    }

    #[test]
    fn test_committed_subdag_empty() {
        let subdag = CommittedSubDag {
            leader: create_test_block_ref(1),
            blocks: vec![],
            timestamp_ms: 1000,
            commit_ref: create_test_commit_ref(1),
            reputation_scores_desc: vec![],
        };
        assert_eq!(subdag.len(), 0);
        assert_eq!(subdag.flatten_transactions().len(), 0);
    }

    #[test]
    fn test_committed_subdag_serialization() {
        let leader = create_test_block_ref(1);
        let commit_ref = create_test_commit_ref(1);
        let block = create_test_verified_block(vec![Transaction::new(vec![1, 2, 3])]);
        let subdag = CommittedSubDag {
            leader,
            blocks: vec![block],
            timestamp_ms: 1234567890,
            commit_ref,
            reputation_scores_desc: vec![(0, 100), (1, 90)],
        };
        let serialized = serde_json::to_string(&subdag).unwrap();
        let deserialized: CommittedSubDag = serde_json::from_str(&serialized).unwrap();
        assert_eq!(subdag.timestamp_ms, deserialized.timestamp_ms);
        assert_eq!(subdag.blocks.len(), deserialized.blocks.len());
        assert_eq!(subdag.leader, deserialized.leader);
        assert_eq!(subdag.commit_ref, deserialized.commit_ref);
        assert_eq!(
            subdag.reputation_scores_desc,
            deserialized.reputation_scores_desc
        );
        // Verify transaction data is preserved
        assert_eq!(
            subdag.flatten_transactions(),
            deserialized.flatten_transactions()
        );
    }

    #[test]
    fn test_committed_subdag_clone() {
        let subdag = CommittedSubDag {
            leader: create_test_block_ref(1),
            blocks: vec![create_test_verified_block(vec![Transaction::new(vec![
                1, 2, 3,
            ])])],
            timestamp_ms: 1000,
            commit_ref: create_test_commit_ref(1),
            reputation_scores_desc: vec![(0, 100)],
        };
        let cloned = subdag.clone();
        assert_eq!(subdag.leader, cloned.leader);
        assert_eq!(subdag.timestamp_ms, cloned.timestamp_ms);
        assert_eq!(subdag.commit_ref, cloned.commit_ref);
        assert_eq!(subdag.blocks.len(), cloned.blocks.len());
        assert_eq!(subdag.reputation_scores_desc, cloned.reputation_scores_desc);
    }

    #[test]
    fn test_verified_block_creation() {
        let transactions = vec![Transaction::new(vec![1, 2, 3])];
        let verified_block = create_test_verified_block(transactions.clone());
        assert_eq!(verified_block.block.transactions().len(), 1);
    }

    #[test]
    fn test_verified_block_serialization() {
        let verified_block = create_test_verified_block(vec![Transaction::new(vec![1, 2, 3])]);
        let serialized = serde_json::to_string(&verified_block).unwrap();
        let deserialized: VerifiedBlock = serde_json::from_str(&serialized).unwrap();
        assert_eq!(
            verified_block.block.transactions().len(),
            deserialized.block.transactions().len()
        );
        // Verify digest is preserved
        assert_eq!(verified_block.digest, deserialized.digest);
        // Verify transaction data is preserved
        assert_eq!(
            verified_block.block.transactions(),
            deserialized.block.transactions()
        );
    }

    #[test]
    fn test_verified_block_clone() {
        let verified_block = create_test_verified_block(vec![Transaction::new(vec![1, 2, 3])]);
        let cloned = verified_block.clone();
        assert_eq!(verified_block.digest, cloned.digest);
        assert_eq!(
            verified_block.block.transactions(),
            cloned.block.transactions()
        );
    }
}
