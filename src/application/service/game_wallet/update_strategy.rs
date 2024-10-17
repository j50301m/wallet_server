use std::fmt::Debug;
use std::sync::Arc;

use bigdecimal::{BigDecimal, Zero};
use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::tracing;

use crate::domain::*;
use crate::enums;

pub struct UpdateStrategyFactory;

impl UpdateStrategyFactory {
    pub fn new(
        wallet_source: &WalletSource,
        wallet_source_repo: Arc<dyn WalletSourceRepositoryTrait>,
        wallet_service: Arc<dyn WalletServiceTrait>,
        rollover_service: Arc<dyn RolloverServiceTrait>,
    ) -> Result<Box<dyn UpdateWalletStrategy>, KgsStatus> {
        let source_enum = enums::WalletSource::from_id(wallet_source.id)?;

        match source_enum {
            enums::WalletSource::Normal => Ok(Box::new(NormalUpdateWalletStrategy::new(
                wallet_service,
                rollover_service,
            ))),
            enums::WalletSource::Bonus => Ok(Box::new(BonusUpdateWalletStrategy::new(
                wallet_service,
                wallet_source_repo,
                rollover_service,
            ))),
        }
    }
}

#[tonic::async_trait]
pub trait UpdateWalletStrategy: Send {
    async fn apply(
        &self,
        wallet_info: &WalletInfo,
        source_txn_id: i64,
        old_amount: BigDecimal,
        new_amount: BigDecimal,
        effective_bet: BigDecimal,
        rollover_rate: BigDecimal,
    ) -> Result<(), KgsStatus>;
}

/// 本金錢包的update策略
#[derive(Debug)]
struct NormalUpdateWalletStrategy {
    wallet_service: Arc<dyn WalletServiceTrait>,
    rollover_service: Arc<dyn RolloverServiceTrait>,
}

/// 本金錢包的update new實作
impl NormalUpdateWalletStrategy {
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

/// 本金錢包的update 實現apply
#[tonic::async_trait]
impl UpdateWalletStrategy for NormalUpdateWalletStrategy {
    #[tracing::instrument]
    async fn apply(
        &self,
        wallet_info: &WalletInfo,
        source_txn_id: i64,
        old_amount: BigDecimal,
        new_amount: BigDecimal,
        effective_bet: BigDecimal,
        rollover_rate: BigDecimal,
    ) -> Result<(), KgsStatus> {
        // 找出需要修改的wallet_txn
        let origin_wallet_txn = self
            .wallet_service
            .get_last_transaction_by_source_id(
                wallet_info.client_id,
                wallet_info.user_id,
                source_txn_id,
            )
            .await?;

        // 轉換wallet_amount 成有正負的數字
        let wallet_txn_amount = match enums::WalletAction::from_i32(origin_wallet_txn.action)? {
            enums::WalletAction::PaymentDeposit | enums::WalletAction::GameDeposit => {
                origin_wallet_txn.change_amount.clone()
            }
            enums::WalletAction::PaymentWithdraw | enums::WalletAction::GameWithdraw => {
                -origin_wallet_txn.change_amount.clone()
            }
        };

        // 檢查金額是否正確
        if &wallet_txn_amount != &old_amount {
            return Err(KgsStatus::GameRollbackAmountError);
        }

        // rollback錢包金額
        let (user_wallet, wallet_txn) = self
            .wallet_service
            .rollback_transaction(&wallet_info, &origin_wallet_txn)
            .await?;

        // rollback流水
        self.rollover_service
            .rollback_rollover(
                user_wallet.id,
                wallet_info,
                wallet_txn.parent_id,
                wallet_txn.id,
                user_wallet.user_id,
            )
            .await?;

        // 計算出要插入的新的動作
        let action = match enums::WalletAction::from_i32(wallet_txn.action)? {
            enums::WalletAction::GameDeposit | enums::WalletAction::GameWithdraw => {
                if new_amount > BigDecimal::zero() {
                    enums::WalletAction::GameDeposit
                } else {
                    enums::WalletAction::GameWithdraw
                }
            }
            _ => {
                if new_amount > BigDecimal::zero() {
                    enums::WalletAction::PaymentDeposit
                } else {
                    enums::WalletAction::PaymentWithdraw
                }
            }
        };

        // 新增 new_amount 的金額到錢包中
        let (user_wallet, wallet_txn) = self
            .wallet_service
            .change_amount(
                wallet_info,
                wallet_txn.id,
                wallet_txn.wallet_source_id,
                new_amount.abs(),
                &action,
            )
            .await?;

        // 修改流水
        self.rollover_service
            .change_rollover(
                user_wallet.id,
                wallet_info,
                wallet_txn.id,
                effective_bet.abs(),
                rollover_rate,
                action,
                user_wallet.user_id,
            )
            .await?;

        Ok(())
    }
}

/// 獎金錢包的update策略
#[derive(Debug)]
struct BonusUpdateWalletStrategy {
    wallet_service: Arc<dyn WalletServiceTrait>,
    wallet_source_repo: Arc<dyn WalletSourceRepositoryTrait>,
    rollover_service: Arc<dyn RolloverServiceTrait>,
}

/// 獎金錢包的update new實作
impl BonusUpdateWalletStrategy {
    #[tracing::instrument]
    pub fn new(
        wallet_service: Arc<dyn WalletServiceTrait>,
        wallet_source_repo: Arc<dyn WalletSourceRepositoryTrait>,
        rollover_service: Arc<dyn RolloverServiceTrait>,
    ) -> Self {
        Self {
            wallet_service,
            wallet_source_repo,
            rollover_service,
        }
    }
}

/// 獎金錢包的update 實現apply
#[tonic::async_trait]
impl UpdateWalletStrategy for BonusUpdateWalletStrategy {
    #[tracing::instrument]
    async fn apply(
        &self,
        wallet_info: &WalletInfo,
        source_txn_id: i64,
        old_amount: BigDecimal,
        new_amount: BigDecimal,
        effective_bet: BigDecimal,
        rollover_rate: BigDecimal,
    ) -> Result<(), KgsStatus> {
        // 找出需要修改的wallet_txn
        let origin_wallet_txn = self
            .wallet_service
            .get_last_transaction_by_source_id(
                wallet_info.client_id,
                wallet_info.user_id,
                source_txn_id,
            )
            .await?;

        // 轉換wallet_amount 成有正負的數字
        let wallet_txn_amount = match enums::WalletAction::from_i32(origin_wallet_txn.action)? {
            enums::WalletAction::PaymentDeposit | enums::WalletAction::GameDeposit => {
                origin_wallet_txn.change_amount.clone()
            }
            enums::WalletAction::PaymentWithdraw | enums::WalletAction::GameWithdraw => {
                -origin_wallet_txn.change_amount.clone()
            }
        };

        // 檢查金額是否正確
        if &wallet_txn_amount != &old_amount {
            return Err(KgsStatus::GameRollbackAmountError);
        }

        // 計算需要補足的金額
        let diff_amount = &new_amount - &old_amount;

        // 如果錢包金額不足，則使用本金錢包來update
        // 如果足夠，則使用獎金錢包來update
        let wallet_info = if !self
            .wallet_service
            .is_wallet_amount_enough(wallet_info, &diff_amount)
            .await?
        {
            let wallet_source = self
                .wallet_source_repo
                .get(wallet_info.wallet_source.id)
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

        // rollback錢包金額
        let (user_wallet, wallet_txn) = self
            .wallet_service
            .rollback_transaction(wallet_info, &origin_wallet_txn)
            .await?;

        // rollback流水
        self.rollover_service
            .rollback_rollover(
                user_wallet.id,
                wallet_info,
                wallet_txn.parent_id,
                wallet_txn.id,
                user_wallet.user_id,
            )
            .await?;

        // 新增一筆update的wallet_txn
        let action = match enums::WalletAction::from_i32(origin_wallet_txn.action)? {
            enums::WalletAction::GameDeposit | enums::WalletAction::GameWithdraw => {
                if new_amount > BigDecimal::zero() {
                    enums::WalletAction::GameDeposit
                } else {
                    enums::WalletAction::GameWithdraw
                }
            }
            _ => {
                if new_amount > BigDecimal::zero() {
                    enums::WalletAction::PaymentDeposit
                } else {
                    enums::WalletAction::PaymentWithdraw
                }
            }
        };
        let (user_wallet, wallet_txn) = self
            .wallet_service
            .change_amount(
                wallet_info,
                wallet_txn.id,
                wallet_txn.transaction_source_id,
                new_amount.abs(),
                &action,
            )
            .await?;

        // 修改流水
        self.rollover_service
            .change_rollover(
                user_wallet.id,
                wallet_info,
                wallet_txn.id,
                effective_bet.abs(),
                rollover_rate,
                action,
                user_wallet.user_id,
            )
            .await?;

        Ok(())
    }
}
