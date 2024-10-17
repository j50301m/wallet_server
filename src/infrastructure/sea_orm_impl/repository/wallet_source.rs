use std::fmt::Debug;

use crate::domain::WalletSourceRepositoryTrait;
use crate::infrastructure::sea_orm_impl::entity::wallet_source;
use database_manager::Context;
use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::{tracing, warn};
use sea_orm::{ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter};

#[derive(Debug)]
pub struct WalletSourceRepository;

#[tonic::async_trait]
impl WalletSourceRepositoryTrait for WalletSourceRepository {
    #[tracing::instrument]
    async fn get(
        &self,
        id: i64,
    ) -> Result<crate::domain::WalletSource, kgs_err::models::status::Status> {
        let cx = Context::current();
        let txn = cx.get::<DatabaseTransaction>().ok_or_else(|| {
            warn!("get database transaction error");
            KgsStatus::InternalServerError
        })?;

        let wallet_source = wallet_source::Entity::find()
            .filter(wallet_source::Column::Id.eq(id))
            .one(txn)
            .await
            .map_err(|e| {
                warn!("get wallet_source error: {:?}", e);
                KgsStatus::InternalServerError
            })?;
        match wallet_source {
            Some(entity) => Ok(entity.into()),
            None => {
                warn!("get wallet_source error: {:?}", KgsStatus::DataNotFound);
                Err(KgsStatus::DataNotFound)
            }
        }
    }
}
