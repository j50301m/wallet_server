use std::fmt::Debug;

use crate::domain::*;
use kgs_err::models::status::Status as KgsStatus;

#[tonic::async_trait]
pub trait RolloverRecordRepositoryTrait: Send + Sync + Debug {
    async fn get(&self, id: i64) -> Result<Option<RolloverRecord>, KgsStatus>;

    async fn get_opt_by_wallet_transaction_id(
        &self,
        wallet_txn_id: i64,
    ) -> Result<Option<RolloverRecord>, KgsStatus>;

    async fn insert(&self, rollover_record: RolloverRecord) -> Result<RolloverRecord, KgsStatus>;
}
