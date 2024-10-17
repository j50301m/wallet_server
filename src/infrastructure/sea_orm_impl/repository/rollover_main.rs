use std::fmt::Debug;

use database_manager::Context;
use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::{tracing, warn};
use sea_orm::*;

use crate::domain::*;
use crate::infrastructure::sea_orm_impl::entity::rollover_main;

#[derive(Debug)]
pub struct RolloverMainRepository;

#[tonic::async_trait]
impl RolloverMainRepositoryTrait for RolloverMainRepository {
    #[tracing::instrument]
    async fn get(&self, wallet_info: &WalletInfo) -> Result<Option<RolloverMain>, KgsStatus> {
        let cx = Context::current();
        let txn = cx.get::<DatabaseTransaction>().ok_or_else(|| {
            warn!("get database transaction error");
            KgsStatus::InternalServerError
        })?;

        rollover_main::Entity::find()
            .filter(rollover_main::Column::ClientId.eq(wallet_info.client_id))
            .filter(rollover_main::Column::UserId.eq(wallet_info.user_id))
            .filter(rollover_main::Column::CurrencyId.eq(wallet_info.currency.id))
            .filter(rollover_main::Column::WalletSourceId.eq(wallet_info.wallet_source.id))
            .one(txn)
            .await
            .map(|entity| entity.map(|entity| entity.into()))
            .map_err(|e| {
                warn!("get rollover_main error: {:?}", e);
                KgsStatus::InternalServerError
            })
    }

    #[tracing::instrument]
    async fn update(&self, rollover_main: RolloverMain) -> Result<RolloverMain, KgsStatus> {
        let cx = Context::current();
        let txn = cx.get::<DatabaseTransaction>().ok_or_else(|| {
            warn!("get database transaction error");
            KgsStatus::InternalServerError
        })?;

        let active_model = rollover_main::ActiveModel::from(rollover_main);
        let entity = active_model.update(txn).await.map_err(|e| {
            warn!("update rollover_main error: {:?}", e);
            KgsStatus::InternalServerError
        })?;

        Ok(entity.into())
    }

    #[tracing::instrument]
    async fn insert(&self, rollover_main: RolloverMain) -> Result<RolloverMain, KgsStatus> {
        let cx = Context::current();
        let txn = cx.get::<DatabaseTransaction>().ok_or_else(|| {
            warn!("get database transaction error");
            KgsStatus::InternalServerError
        })?;

        let active_model = rollover_main::ActiveModel::from(rollover_main);
        let entity = active_model.insert(txn).await.map_err(|e| {
            warn!("insert rollover_main error: {:?}", e);
            KgsStatus::InternalServerError
        })?;

        Ok(entity.into())
    }
}
