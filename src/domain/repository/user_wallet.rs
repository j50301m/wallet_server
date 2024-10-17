use std::fmt::Debug;

use crate::domain::{self, aggregate, vo};
use kgs_err::models::status::Status as KgsStatus;

#[tonic::async_trait]
pub trait UserWalletRepositoryTrait: Send + Sync + Debug {
    async fn get_user_wallets_with_rollover(
        &self,
        select_query: vo::SelectWalletsQuery,
    ) -> Result<Vec<aggregate::UserWalletWithRollover>, KgsStatus>;

    async fn get(
        &self,
        wallet_info: &vo::WalletInfo,
    ) -> Result<Option<domain::UserWallet>, KgsStatus>;

    async fn insert(
        &self,
        user_wallet: domain::UserWallet,
    ) -> Result<domain::UserWallet, KgsStatus>;

    async fn update(
        &self,
        user_wallet: domain::UserWallet,
    ) -> Result<domain::UserWallet, KgsStatus>;
}
