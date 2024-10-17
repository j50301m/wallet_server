use std::{fmt::Debug, sync::Arc};

use crate::domain::*;
use crate::enums;
use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::tracing;

pub struct RollbackWalletStrategyFactory;

impl RollbackWalletStrategyFactory {
    pub fn new(
        wallet_source: &WalletSource,
        currency_service: Arc<dyn CurrencyServiceTrait>,
        wallet_source_repo: Arc<dyn WalletSourceRepositoryTrait>,
        wallet_service: Arc<dyn WalletServiceTrait>,
        rollover_service: Arc<dyn RolloverServiceTrait>,
    ) -> Result<Box<dyn RollbackWalletStrategy + Send>, KgsStatus> {
        let source_enum = enums::WalletSource::from_id(wallet_source.id)?;

        match source_enum {
            enums::WalletSource::Normal => Ok(Box::new(RollbackNormalWalletStrategy::new(
                currency_service,
                wallet_service,
                rollover_service,
            ))),
            enums::WalletSource::Bonus => Ok(Box::new(RollbackBonusWalletStrategy::new(
                wallet_service,
                rollover_service,
                wallet_source_repo,
                currency_service,
            ))),
        }
    }
}

#[tonic::async_trait]
pub trait RollbackWalletStrategy {
    async fn apply(
        &self,
        client_id: i64,
        user_id: i64,
        wallet_source: WalletSource,
        source_txn_id: i64,
    ) -> Result<(UserWallet, RolloverMain), KgsStatus>;
}

/// 本金錢包的rollback策略
#[derive(Debug)]
struct RollbackNormalWalletStrategy {
    currency_service: Arc<dyn CurrencyServiceTrait>,
    wallet_service: Arc<dyn WalletServiceTrait>,
    rollover_service: Arc<dyn RolloverServiceTrait>,
}

impl RollbackNormalWalletStrategy {
    #[tracing::instrument]
    pub fn new(
        currency_service: Arc<dyn CurrencyServiceTrait>,
        wallet_service: Arc<dyn WalletServiceTrait>,
        rollover_service: Arc<dyn RolloverServiceTrait>,
    ) -> Self {
        Self {
            currency_service,
            wallet_service,
            rollover_service,
        }
    }
}

#[tonic::async_trait]
impl RollbackWalletStrategy for RollbackNormalWalletStrategy {
    #[tracing::instrument]
    async fn apply(
        &self,
        client_id: i64,
        user_id: i64,
        wallet_source: WalletSource,
        source_txn_id: i64,
    ) -> Result<(UserWallet, RolloverMain), KgsStatus> {
        // 獲取最後一筆交易紀錄
        let last_wallet_txn = self
            .wallet_service
            .get_last_transaction_by_source_id(client_id, user_id, source_txn_id)
            .await?;

        // 檢查幣別
        let currency = self
            .currency_service
            .get_enable_currency_by_id(client_id, last_wallet_txn.currency_id)
            .await?;

        // 組裝WalletInfo
        let wallet_info = WalletInfo {
            client_id: client_id,
            user_id: user_id,
            currency,
            wallet_source,
        };

        // rollback錢包金額
        let (user_wallet, wallet_txn) = self
            .wallet_service
            .rollback_transaction(&wallet_info, &last_wallet_txn)
            .await?;

        // rollback流水
        let (rollover_main, _rollover_record) = self
            .rollover_service
            .rollback_rollover(
                user_wallet.id,
                &wallet_info,
                wallet_txn.parent_id,
                wallet_txn.id,
                user_id,
            )
            .await?;

        Ok((user_wallet, rollover_main))
    }
}

/// 獎金錢包的rollback策略
#[derive(Debug)]
struct RollbackBonusWalletStrategy {
    wallet_service: Arc<dyn WalletServiceTrait>,
    rollover_service: Arc<dyn RolloverServiceTrait>,
    wallet_source_repo: Arc<dyn WalletSourceRepositoryTrait>,
    currency_service: Arc<dyn CurrencyServiceTrait>,
}

impl RollbackBonusWalletStrategy {
    #[tracing::instrument]
    pub fn new(
        wallet_service: Arc<dyn WalletServiceTrait>,
        rollover_service: Arc<dyn RolloverServiceTrait>,
        wallet_source_repo: Arc<dyn WalletSourceRepositoryTrait>,
        currency_service: Arc<dyn CurrencyServiceTrait>,
    ) -> Self {
        Self {
            wallet_service,
            rollover_service,
            wallet_source_repo,
            currency_service,
        }
    }
}

#[tonic::async_trait]
impl RollbackWalletStrategy for RollbackBonusWalletStrategy {
    #[tracing::instrument]
    async fn apply(
        &self,
        client_id: i64,
        user_id: i64,
        wallet_source: WalletSource,
        source_txn_id: i64,
    ) -> Result<(UserWallet, RolloverMain), KgsStatus> {
        // 獲取最後一筆交易紀錄
        let last_wallet_txn = self
            .wallet_service
            .get_last_transaction_by_source_id(client_id, user_id, source_txn_id)
            .await?;

        // 檢查幣別
        let currency = self
            .currency_service
            .get_enable_currency_by_id(client_id, last_wallet_txn.currency_id)
            .await?;

        // 組裝WalletInfo
        let wallet_info = WalletInfo {
            client_id: client_id,
            user_id: user_id,
            currency,
            wallet_source,
        };

        // 如果錢不夠扣除本金錢包
        let wallet_info = if !self
            .wallet_service
            .is_wallet_amount_enough(&wallet_info, &last_wallet_txn.change_amount)
            .await?
        {
            let wallet_source = self
                .wallet_source_repo
                .get(enums::WalletSource::Normal.to_id())
                .await?;
            WalletInfo {
                client_id: client_id,
                user_id: user_id,
                currency: wallet_info.currency,
                wallet_source,
            }
        } else {
            wallet_info
        };

        // rollback錢包金額
        let (user_wallet, wallet_txn) = self
            .wallet_service
            .rollback_transaction(&wallet_info, &last_wallet_txn)
            .await?;

        // rollback流水
        let (rollover_main, _rollover_record) = self
            .rollover_service
            .rollback_rollover(
                user_wallet.id,
                &wallet_info,
                wallet_txn.parent_id,
                wallet_txn.id,
                user_id,
            )
            .await?;

        Ok((user_wallet, rollover_main))
    }
}
