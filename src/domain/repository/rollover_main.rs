use std::fmt::Debug;

use crate::domain::{self, vo};
use kgs_err::models::status::Status as KgsStatus;

#[tonic::async_trait]
pub trait RolloverMainRepositoryTrait: Debug + Sync + Send {
    async fn get(
        &self,
        wallet_info: &vo::WalletInfo,
    ) -> Result<Option<domain::RolloverMain>, KgsStatus>;

    async fn update(
        &self,
        wallet_info: domain::RolloverMain,
    ) -> Result<domain::RolloverMain, KgsStatus>;

    async fn insert(
        &self,
        rollover_main: domain::RolloverMain,
    ) -> Result<domain::RolloverMain, KgsStatus>;
}
