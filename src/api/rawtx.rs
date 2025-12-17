use jsonrpsee::{
    core::{RpcResult, SubscriptionResult},
    proc_macros::rpc,
};
/// Bytes type alias for raw transaction data.
/// Using Vec<u8> for better serialization support without external dependencies.
pub type Bytes = Vec<u8>;
/// trait interface for a custom rpc namespace: `txpool`
///
/// This defines an additional namespace where all methods are configured as trait functions.
#[rpc(server, client, namespace = "rawtx")]
pub trait RawTransactionApi {
    /// Send a raw transaction to the network.
    #[method(name = "sendRawTransactionAsync")]
    async fn send_raw_transaction_async(&self, bytes: Bytes) -> RpcResult<()>;
    /// Send multiple raw transactions to the network in a batch.
    #[method(name = "sendRawTransactionsAsync")]
    async fn send_raw_transactions_async(&self, transactions: Vec<Bytes>) -> RpcResult<()>;
    /// Creates a subscription that listens to all raw transactions when it comes to rpc server.
    #[subscription(name = "subscribeRawTransactions", item = Vec<Bytes>)]
    fn subscribe_raw_transactions(&self) -> SubscriptionResult;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_type_alias() {
        let bytes: Bytes = vec![1, 2, 3, 4, 5];
        assert_eq!(bytes.len(), 5);
        assert_eq!(bytes[0], 1);
    }

    #[test]
    fn test_bytes_empty() {
        let bytes: Bytes = vec![];
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_bytes_operations() {
        let mut bytes: Bytes = vec![1, 2, 3];
        bytes.push(4);
        assert_eq!(bytes.len(), 4);
        assert_eq!(bytes, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_bytes_serialization() {
        let bytes: Bytes = vec![1, 2, 3, 4, 5];
        let serialized = serde_json::to_string(&bytes).unwrap();
        let deserialized: Bytes = serde_json::from_str(&serialized).unwrap();
        assert_eq!(bytes, deserialized);
    }

    #[test]
    fn test_bytes_vec_of_bytes() {
        let transactions: Vec<Bytes> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(transactions.len(), 3);
        assert_eq!(transactions[0], vec![1, 2, 3]);
    }

    #[test]
    fn test_bytes_large_data() {
        let bytes: Bytes = vec![0u8; 1000];
        assert_eq!(bytes.len(), 1000);
    }
}
