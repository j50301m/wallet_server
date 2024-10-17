use std::fmt::Debug;
use std::sync::Arc;

use bigdecimal::{BigDecimal, Zero};
use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::tracing;

use crate::domain::*;
use crate::enums;

pub struct RollbackStrategyFactory;

impl RollbackStrategyFactory {
    pub fn new(
        wallet_source: &WalletSource,
        wallet_source_repo: Arc<dyn WalletSourceRepositoryTrait>,
        wallet_service: Arc<dyn WalletServiceTrait>,
        rollover_service: Arc<dyn RolloverServiceTrait>,
    ) -> Result<Box<dyn RollbackWalletStrategy>, KgsStatus> {
        let source_enum = enums::WalletSource::from_id(wallet_source.id)?;

        match source_enum {
            enums::WalletSource::Normal => Ok(Box::new(NormalRollbackWalletStrategy::new(
                wallet_service,
                rollover_service,
            ))),
            enums::WalletSource::Bonus => Ok(Box::new(BonusRollbackWalletStrategy::new(
                wallet_service,
                rollover_service,
                wallet_source_repo,
            ))),
        }
    }
}

#[tonic::async_trait]
pub trait RollbackWalletStrategy: Send {
    async fn apply(
        &self,
        wallet_info: &WalletInfo,
        source_txn_ids: &[i64],
    ) -> Result<(), KgsStatus>;
}

/// 本金錢包的rollback策略
#[derive(Debug)]
struct NormalRollbackWalletStrategy {
    wallet_service: Arc<dyn WalletServiceTrait>,
    rollover_service: Arc<dyn RolloverServiceTrait>,
}

/// 本金錢包的rollback策略 new方法
impl NormalRollbackWalletStrategy {
    #[tracing::instrument]
    pub fn new(
        wallet_service: Arc<dyn WalletServiceTrait>,
        rollover_service: Arc<dyn RolloverServiceTrait>,
    ) -> Self {
        Self {
            wallet_service,
            rollover_service,
        }
    }
}

#[tonic::async_trait]
/// 本金錢包的rollback策略 實際執行方法
impl RollbackWalletStrategy for NormalRollbackWalletStrategy {
    #[tracing::instrument]
    async fn apply(
        &self,
        wallet_info: &WalletInfo,
        source_txn_ids: &[i64],
    ) -> Result<(), KgsStatus> {
        // 把所有需要rollback的最後一筆交易紀錄找出來
        // 如果有任何一筆單筆交易紀錄找不到，就回傳錯誤 不進行rollback
        let mut wallet_txn_list = vec![];
        for source_txn in source_txn_ids {
            let wallet_txn = self
                .wallet_service
                .get_last_transaction_by_source_id(
                    wallet_info.client_id,
                    wallet_info.user_id,
                    *source_txn,
                )
                .await?;
            wallet_txn_list.push(wallet_txn);
        }

        // 對每筆交易紀錄進行rollback
        for wallet_txn in wallet_txn_list.iter() {
            // rollback 錢包金額
            let (user_wallet, wallet_txn) = self
                .wallet_service
                .rollback_transaction(wallet_info, wallet_txn)
                .await?;

            // rollback 流水
            self.rollover_service
                .rollback_rollover(
                    user_wallet.id,
                    wallet_info,
                    wallet_txn.parent_id,
                    wallet_txn.id,
                    user_wallet.user_id,
                )
                .await?;
        }

        Ok(())
    }
}

/// 獎金錢包的rollback策略
#[derive(Debug)]
struct BonusRollbackWalletStrategy {
    wallet_service: Arc<dyn WalletServiceTrait>,
    rollover_service: Arc<dyn RolloverServiceTrait>,
    wallet_source_repo: Arc<dyn WalletSourceRepositoryTrait>,
}

/// 獎金錢包的rollback策略 new方法
impl BonusRollbackWalletStrategy {
    #[tracing::instrument]
    pub fn new(
        wallet_service: Arc<dyn WalletServiceTrait>,
        rollover_service: Arc<dyn RolloverServiceTrait>,
        wallet_source_repo: Arc<dyn WalletSourceRepositoryTrait>,
    ) -> Self {
        Self {
            wallet_service,
            rollover_service,
            wallet_source_repo,
        }
    }
}

/// 獎金錢包的rollback策略 實際執行方法
#[tonic::async_trait]
impl RollbackWalletStrategy for BonusRollbackWalletStrategy {
    #[tracing::instrument]
    async fn apply(
        &self,
        wallet_info: &WalletInfo,
        source_txn_ids: &[i64],
    ) -> Result<(), KgsStatus> {
        // 把所有需要rollback的最後一筆交易紀錄找出來
        // 如果有任何一筆單筆交易紀錄找不到，就回傳錯誤 不進行rollback
        let mut wallet_txn_list = vec![];
        for source_txn in source_txn_ids {
            let wallet_txn = self
                .wallet_service
                .get_last_transaction_by_source_id(
                    wallet_info.client_id,
                    wallet_info.user_id,
                    *source_txn,
                )
                .await?;
            wallet_txn_list.push(wallet_txn);
        }

        // 計算全部需要rollback的金額是否足夠
        let mut need_rollback_amount = BigDecimal::zero();
        for txn in wallet_txn_list.iter() {
            match enums::WalletAction::from_i32(txn.action)? {
                enums::WalletAction::GameDeposit | enums::WalletAction::PaymentDeposit => {
                    need_rollback_amount -= &txn.change_amount;
                }
                enums::WalletAction::GameWithdraw | enums::WalletAction::PaymentWithdraw => {
                    need_rollback_amount += &txn.change_amount;
                }
            }
        }

        // 如果金額不足 則用本金錢包進行rollback
        // 足夠則用獎金錢包進行rollback
        let wallet_info = if !self
            .wallet_service
            .is_wallet_amount_enough(wallet_info, &need_rollback_amount)
            .await?
        {
            let wallet_source = self
                .wallet_source_repo
                .get(enums::WalletSource::Normal.to_id())
                .await?;
            &WalletInfo {
                client_id: wallet_info.client_id,
                user_id: wallet_info.user_id,
                currency: wallet_info.currency.clone(),
                wallet_source,
            }
        } else {
            wallet_info
        };

        // 對每一筆交易紀錄進行rollback
        for wallet_txn in wallet_txn_list.iter() {
            // rollback 錢包金額
            let (user_wallet, wallet_txn) = self
                .wallet_service
                .rollback_transaction(wallet_info, wallet_txn)
                .await?;

            // rollback 流水
            self.rollover_service
                .rollback_rollover(
                    user_wallet.id,
                    wallet_info,
                    wallet_txn.parent_id,
                    wallet_txn.id,
                    user_wallet.user_id,
                )
                .await?;
        }

        Ok(())
    }
}
