use std::{fmt::Debug, sync::Arc};

use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::{tracing, warn};

use super::proto_trait::GetWalletInfoTrait;
use crate::domain;

#[tonic::async_trait]
pub trait WalletMapperTrait: Send + Sync + Debug {
    async fn to_wallet_info(
        &self,
        proto: &dyn GetWalletInfoTrait,
    ) -> Result<domain::WalletInfo, KgsStatus>;
    fn to_wallet_proto(
        &self,
        user_wallet: domain::UserWallet,
        rollover_main: domain::RolloverMain,
    ) -> Result<protos::player_wallet::WalletModel, KgsStatus>;
}

#[derive(Debug)]
pub struct WalletMapper {
    currency_service: Arc<dyn domain::CurrencyServiceTrait>,
    wallet_source_repo: Arc<dyn domain::WalletSourceRepositoryTrait>,
}

impl WalletMapper {
    pub fn new(
        currency_service: Arc<dyn domain::CurrencyServiceTrait>,
        wallet_source_repo: Arc<dyn domain::WalletSourceRepositoryTrait>,
    ) -> Self {
        Self {
            currency_service,
            wallet_source_repo,
        }
    }
}

#[tonic::async_trait]
impl WalletMapperTrait for WalletMapper {
    #[tracing::instrument]
    async fn to_wallet_info(
        &self,
        proto: &dyn GetWalletInfoTrait,
    ) -> Result<domain::WalletInfo, KgsStatus> {
        let currency = self
            .currency_service
            .get_enable_currency(proto.get_client_id(), proto.get_currency_name())
            .await
            .map_err(|e| {
                warn!("get enable currency error: {}", e);
                KgsStatus::InternalServerError
            })?;
        let wallet_source = self
            .wallet_source_repo
            .get(proto.get_wallet_source_id())
            .await
            .map_err(|e| {
                warn!("get wallet source error: {}", e);
                KgsStatus::InternalServerError
            })?;

        Ok(domain::WalletInfo {
            client_id: proto.get_client_id(),
            user_id: proto.get_user_id(),
            currency,
            wallet_source,
        })
    }

    #[tracing::instrument]
    fn to_wallet_proto(
        &self,
        user_wallet: domain::UserWallet,
        rollover_main: domain::RolloverMain,
    ) -> Result<protos::player_wallet::WalletModel, KgsStatus> {
        Ok(protos::player_wallet::WalletModel {
            client_id: user_wallet.client_id,
            user_id: user_wallet.user_id,
            currency: user_wallet.currency_name,
            wallet_source_id: user_wallet.wallet_source_id,
            wallet_source_name: user_wallet.wallet_source_name,
            amount: user_wallet.amount.to_string(),
            requirement_rollover: rollover_main.requirement_rollover.to_string(),
            achievement_rollover: rollover_main.achievement_rollover.to_string(),
        })
    }
}
