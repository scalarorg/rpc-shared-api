use crate::types::{AuthorityIndex, BlockRef, CommitRef};

/// A helper structure for working with committed subdags containing generic transaction types.
/// This type is not serializable by design - consumers should convert to their own types
/// or use `CommittedSubDag` for serialization.
#[derive(Debug, Clone)]
pub struct MysticetiCommittedSubdag<Transaction> {
    pub leader: BlockRef,
    pub transactions: Vec<Transaction>,
    pub timestamp_ms: u64,
    pub commit_ref: CommitRef,
    pub reputation_scores_desc: Vec<(AuthorityIndex, u64)>,
}

/// Serialize a batch of raw transaction bytes to JSON.
/// Consumers can use this to create SubscriptionMessage in their own code.
pub fn serialize_transactions(batch: Vec<Vec<u8>>) -> Result<String, serde_json::Error> {
    serde_json::to_string(&batch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{BlockRef, CommitRef};

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

    #[test]
    fn test_mysticeti_committed_subdag_creation() {
        let leader = create_test_block_ref(1);
        let transactions = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let commit_ref = create_test_commit_ref(1);
        let reputation_scores = vec![(0, 100), (1, 90)];

        let subdag = MysticetiCommittedSubdag {
            leader,
            transactions: transactions.clone(),
            timestamp_ms: 1234567890,
            commit_ref,
            reputation_scores_desc: reputation_scores.clone(),
        };

        assert_eq!(subdag.leader, leader);
        assert_eq!(subdag.transactions, transactions);
        assert_eq!(subdag.timestamp_ms, 1234567890);
        assert_eq!(subdag.commit_ref, commit_ref);
        assert_eq!(subdag.reputation_scores_desc, reputation_scores);
    }

    #[test]
    fn test_mysticeti_committed_subdag_clone() {
        let leader = create_test_block_ref(1);
        let transactions = vec![vec![1, 2, 3]];
        let commit_ref = create_test_commit_ref(1);

        let subdag1 = MysticetiCommittedSubdag {
            leader,
            transactions: transactions.clone(),
            timestamp_ms: 1000,
            commit_ref,
            reputation_scores_desc: vec![],
        };

        let subdag2 = subdag1.clone();
        assert_eq!(subdag1.leader, subdag2.leader);
        assert_eq!(subdag1.transactions, subdag2.transactions);
        assert_eq!(subdag1.timestamp_ms, subdag2.timestamp_ms);
    }

    #[test]
    fn test_serialize_transactions_empty() {
        let batch: Vec<Vec<u8>> = vec![];
        let result = serialize_transactions(batch);
        assert!(result.is_ok());
        let json = result.unwrap();
        assert_eq!(json, "[]");
    }

    #[test]
    fn test_serialize_transactions_single() {
        let batch = vec![vec![1, 2, 3, 4, 5]];
        let result = serialize_transactions(batch);
        assert!(result.is_ok());
        let json = result.unwrap();
        // Should be valid JSON array
        assert!(json.starts_with('['));
        assert!(json.ends_with(']'));
    }

    #[test]
    fn test_serialize_transactions_multiple() {
        let batch = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = serialize_transactions(batch);
        assert!(result.is_ok());
        let json = result.unwrap();
        // Should be valid JSON array
        assert!(json.starts_with('['));
        assert!(json.ends_with(']'));
        // Verify it can be deserialized
        let deserialized: Vec<Vec<u8>> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.len(), 3);
        assert_eq!(deserialized[0], vec![1, 2, 3]);
        assert_eq!(deserialized[1], vec![4, 5, 6]);
        assert_eq!(deserialized[2], vec![7, 8, 9]);
    }

    #[test]
    fn test_serialize_transactions_large_data() {
        let batch = vec![vec![0u8; 1000], vec![255u8; 500]];
        let result = serialize_transactions(batch.clone());
        assert!(result.is_ok());
        let json = result.unwrap();
        let deserialized: Vec<Vec<u8>> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.len(), 2);
        assert_eq!(deserialized[0].len(), 1000);
        assert_eq!(deserialized[1].len(), 500);
    }
}
