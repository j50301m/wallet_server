use std::fmt::Debug;

use database_manager::Context;
use sea_orm::*;

use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::{tracing, warn};

use crate::domain::*;
use crate::infrastructure::sea_orm_impl;

#[derive(Debug)]
pub struct RolloverRecordRepository;

#[tonic::async_trait]
impl RolloverRecordRepositoryTrait for RolloverRecordRepository {
    #[tracing::instrument]
    async fn get(&self, id: i64) -> Result<Option<RolloverRecord>, KgsStatus> {
        let cx = Context::current();
        let txn = cx.get::<DatabaseTransaction>().ok_or_else(|| {
            warn!("get database transaction error");
            KgsStatus::InternalServerError
        })?;
        sea_orm_impl::entity::rollover_record::Entity::find_by_id(id)
            .one(txn)
            .await
            .map(|entity| entity.map(|entity| entity.into()))
            .map_err(|e| {
                warn!("get rollover_record error: {:?}", e);
                KgsStatus::InternalServerError
            })
    }

    #[tracing::instrument]
    async fn insert(&self, rollover_record: RolloverRecord) -> Result<RolloverRecord, KgsStatus> {
        let cx = Context::current();
        let txn = cx.get::<DatabaseTransaction>().ok_or_else(|| {
            warn!("get database transaction error");
            KgsStatus::InternalServerError
        })?;

        let active_model =
            sea_orm_impl::entity::rollover_record::ActiveModel::from(rollover_record);
        let entity = active_model.insert(txn).await.map_err(|e| {
            warn!("insert rollover_record error: {:?}", e);
            KgsStatus::InternalServerError
        })?;

        Ok(entity.into())
    }

    #[tracing::instrument]
    async fn get_opt_by_wallet_transaction_id(
        &self,
        wallet_txn_id: i64,
    ) -> Result<Option<RolloverRecord>, KgsStatus> {
        let cx = Context::current();
        let txn = cx.get::<DatabaseTransaction>().ok_or_else(|| {
            warn!("get database transaction error");
            KgsStatus::InternalServerError
        })?;

        sea_orm_impl::entity::rollover_record::Entity::find()
            .filter(sea_orm_impl::entity::rollover_record::Column::WalletTxnId.eq(wallet_txn_id))
            .one(txn)
            .await
            .map(|entity| entity.map(|entity| entity.into()))
            .map_err(|e| {
                warn!("get rollover_record error: {:?}", e);
                KgsStatus::InternalServerError
            })
    }
}
