use std::fmt::Debug;

use crate::domain;
use kgs_err::models::status::Status as KgsStatus;

#[tonic::async_trait]
pub trait WalletSourceRepositoryTrait: Sync + Send + Debug {
    async fn get(&self, id: i64) -> Result<domain::WalletSource, KgsStatus>;
}
