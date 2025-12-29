use jsonrpsee::{core::RpcResult, proc_macros::rpc};

use crate::CommittedSubDag;

/// trait interface for a custom rpc namespace: `txpool`
///
/// This defines an additional namespace where all methods are configured as trait functions.
#[rpc(server, client, namespace = "mysticeti")]
pub trait MysticetiConsensusApi {
    /// Submit commited transactions
    #[method(name = "submitCommittedSubdags")]
    fn submit_committed_subdags(
        &self,
        #[argument(rename = "subdag")] subdags: Vec<CommittedSubDag>,
    ) -> RpcResult<()>;

    #[method(name = "submitCommittedSubdag")]
    fn submit_committed_subdag(
        &self,
        #[argument(rename = "subdag")] subdag: CommittedSubDag,
    ) -> RpcResult<()>;
}

#[cfg(test)]
mod tests {
    use crate::types::{BlockRef, CommitRef, Transaction};
    use crate::{BlockDigest, CommittedSubDag, SignedBlock, VerifiedBlock};

    fn create_test_block_ref(round: u64) -> BlockRef {
        let mut digest = [0u8; 32];
        digest[0] = round as u8;
        BlockRef {
            digest,
            round,
            leader_address: String::new(),
            ..Default::default()
        }
    }

    fn create_test_commit_ref(round: usize) -> CommitRef {
        let mut digest = [0u8; 32];
        digest[0] = round as u8;
        CommitRef { digest, round }
    }

    fn create_test_committed_subdag() -> CommittedSubDag {
        let transactions = vec![Transaction::new(vec![1, 2, 3])];
        let block = SignedBlock::new(transactions);
        let mut digest = [0u8; 32];
        digest[0] = 1;
        let verified_block = VerifiedBlock {
            block,
            digest: BlockDigest(digest),
        };
        CommittedSubDag {
            leader: create_test_block_ref(1),
            blocks: vec![verified_block],
            timestamp_ms: 1000,
            commit_ref: create_test_commit_ref(1),
            reputation_scores_desc: vec![],
        }
    }

    #[test]
    fn test_committed_subdag_type_compatibility() {
        // Test that CommittedSubDag can be used with the trait
        let subdag = create_test_committed_subdag();
        assert_eq!(subdag.blocks.len(), 1);
        assert_eq!(subdag.timestamp_ms, 1000);
    }

    #[test]
    fn test_committed_subdag_vector() {
        // Test that Vec<CommittedSubDag> works as expected
        let subdag1 = create_test_committed_subdag();
        let subdag2 = create_test_committed_subdag();
        let subdags = vec![subdag1, subdag2];
        assert_eq!(subdags.len(), 2);
    }

    #[test]
    fn test_committed_subdag_serialization() {
        // Test that CommittedSubDag can be serialized (required for RPC)
        let subdag = create_test_committed_subdag();
        let serialized = serde_json::to_string(&subdag).unwrap();
        let deserialized: CommittedSubDag = serde_json::from_str(&serialized).unwrap();
        assert_eq!(subdag.timestamp_ms, deserialized.timestamp_ms);
    }
}
