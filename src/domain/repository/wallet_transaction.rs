use std::fmt::Debug;

use crate::domain;
use kgs_err::models::status::Status as KgsStatus;

#[tonic::async_trait]
pub trait WalletTransactionRepositoryTrait: Sync + Send + Debug {
    async fn insert(
        &self,
        wallet_txn: domain::WalletTransaction,
    ) -> Result<domain::WalletTransaction, KgsStatus>;

    async fn get_list_by_transaction_source_id(
        &self,
        client_id: i64,
        user_id: i64,
        source_txn_id: i64,
    ) -> Result<Vec<domain::WalletTransaction>, KgsStatus>;
}
