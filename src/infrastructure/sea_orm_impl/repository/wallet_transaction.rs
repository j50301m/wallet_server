use std::fmt::Debug;

use database_manager::Context;
use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::{tracing, warn};
use sea_orm::*;

use crate::domain;
use crate::domain::WalletTransactionRepositoryTrait;
use crate::infrastructure::sea_orm_impl::entity::wallet_transaction;

#[derive(Debug)]
pub struct WalletTransactionRepository;

#[tonic::async_trait]
impl WalletTransactionRepositoryTrait for WalletTransactionRepository {
    #[tracing::instrument]
    async fn insert(
        &self,
        wallet_txn: domain::WalletTransaction,
    ) -> Result<domain::WalletTransaction, KgsStatus> {
        let cx = Context::current();
        let txn = cx.get::<DatabaseTransaction>().ok_or_else(|| {
            warn!("get database transaction error");
            KgsStatus::InternalServerError
        })?;

        let active_model = wallet_transaction::ActiveModel::from(wallet_txn);

        active_model
            .insert(txn)
            .await
            .map(|model| model.into())
            .map_err(|err| {
                warn!("insert wallet transaction failed: {:?}", err);
                KgsStatus::InternalServerError
            })
    }

    #[tracing::instrument]
    async fn get_list_by_transaction_source_id(
        &self,
        client_id: i64,
        user_id: i64,
        source_txn_id: i64,
    ) -> Result<Vec<domain::WalletTransaction>, KgsStatus> {
        let cx = Context::current();
        let txn = cx.get::<DatabaseTransaction>().ok_or_else(|| {
            warn!("get database transaction error");
            KgsStatus::InternalServerError
        })?;

        wallet_transaction::Entity::find()
            .filter(wallet_transaction::Column::ClientId.eq(client_id))
            .filter(wallet_transaction::Column::UserId.eq(user_id))
            .filter(wallet_transaction::Column::TransactionSourceId.eq(source_txn_id))
            .all(txn)
            .await
            .map(|models| models.into_iter().map(|model| model.into()).collect())
            .map_err(|err| {
                warn!("get wallet transaction list failed: {:?}", err);
                KgsStatus::InternalServerError
            })
    }
}
