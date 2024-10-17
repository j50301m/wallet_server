use std::fmt::Debug;
use std::sync::Arc;

use bigdecimal::{BigDecimal, Zero};
use database_manager::transactional;
use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::tracing;
use protos::game_wallet::*;

use super::rollback_strategy::RollbackStrategyFactory;
use super::update_strategy::UpdateStrategyFactory;
use crate::application::dto::{self, *};
use crate::domain;
use crate::enums::WalletAction;

#[derive(Debug)]
pub struct GameWalletService {
    wallet_source_repo: Arc<dyn domain::WalletSourceRepositoryTrait>,
    wallet_service: Arc<dyn domain::WalletServiceTrait>,
    rollover_service: Arc<dyn domain::RolloverServiceTrait>,
    mapper: Arc<dyn dto::WalletMapperTrait>,
}

impl GameWalletService {
    pub fn new(
        wallet_source_repo: Arc<dyn domain::WalletSourceRepositoryTrait>,
        wallet_service: Arc<dyn domain::WalletServiceTrait>,
        rollover_service: Arc<dyn domain::RolloverServiceTrait>,
        mapper: Arc<dyn dto::WalletMapperTrait>,
    ) -> Self {
        Self {
            wallet_source_repo,
            wallet_service,
            rollover_service,
            mapper,
        }
    }
}

impl GameWalletService {
    #[tracing::instrument]
    #[transactional(SeaOrmPostgres)]
    pub async fn get_balance(&self, payload: BalanceRequest) -> Result<BalanceResponse, KgsStatus> {
        let wallet_info = self.mapper.to_wallet_info(&payload).await?;

        let wallet_entity = self
            .wallet_service
            .get_or_create_new_one(&wallet_info)
            .await?;

        Ok(BalanceResponse {
            balance: wallet_entity.amount.to_string(),
        })
    }

    #[tracing::instrument]
    #[transactional(SeaOrmPostgres)]
    pub async fn deposit(&self, payload: DepositRequest) -> Result<DepositResponse, KgsStatus> {
        let wallet_info = self.mapper.to_wallet_info(&payload).await?;
        let amount = payload.get_amount()?;
        let effective_bet = payload.get_effective_bet()?;
        let rollover_rate = payload.get_rollover_rate()?;

        // 錢包上分
        let (user_wallet, wallet_txn) = self
            .wallet_service
            .change_amount(
                &wallet_info,
                0,
                payload.transaction_id,
                amount,
                &WalletAction::GameDeposit,
            )
            .await?;

        // 修改流水
        let (_rollover_main, _rollover_record) = self
            .rollover_service
            .change_rollover(
                user_wallet.id,
                &wallet_info,
                wallet_txn.id,
                effective_bet,
                rollover_rate,
                WalletAction::GameDeposit,
                wallet_info.user_id,
            )
            .await?;

        // TODO: 如果是獎金錢包 需要判斷流水是否打滿 然後做對應的處理

        Ok(DepositResponse {
            balance: user_wallet.amount.to_string(),
        })
    }

    #[tracing::instrument]
    pub async fn withdraw(&self, payload: WithdrawRequest) -> Result<WithdrawResponse, KgsStatus> {
        let wallet_info = self.mapper.to_wallet_info(&payload).await?;
        let amount = payload.get_amount()?;

        // 檢查餘額是否足夠
        if !self
            .wallet_service
            .is_wallet_amount_enough(&wallet_info, &amount)
            .await?
        {
            return Err(KgsStatus::WalletAmountNotEnough);
        }

        // 錢包下分
        let (user_wallet, wallet_txn) = self
            .wallet_service
            .change_amount(
                &wallet_info,
                0,
                payload.transaction_id,
                amount.clone(),
                &WalletAction::GameWithdraw,
            )
            .await?;

        // 修改流水
        let (_rollover_main, _rollover_record) = self
            .rollover_service
            .change_rollover(
                user_wallet.id,
                &wallet_info,
                wallet_txn.id,
                amount,
                BigDecimal::zero(),
                WalletAction::GameWithdraw,
                wallet_info.user_id,
            )
            .await?;

        Ok(WithdrawResponse {
            balance: user_wallet.amount.to_string(),
        })
    }

    #[tracing::instrument]
    pub async fn rollback(&self, payload: RollbackRequest) -> Result<RollbackResponse, KgsStatus> {
        let wallet_info = self.mapper.to_wallet_info(&payload).await?;

        // 創建rollback策略
        let strategy = RollbackStrategyFactory::new(
            &wallet_info.wallet_source,
            self.wallet_source_repo.clone(),
            self.wallet_service.clone(),
            self.rollover_service.clone(),
        )?;

        // 執行rollback策略 對應獎金錢包與本金錢包 有不同的方法
        strategy
            .apply(&wallet_info, &payload.transaction_ids)
            .await?;

        // 獲取傳入錢包的最新餘額
        let user_wallet = self
            .wallet_service
            .get_or_create_new_one(&wallet_info)
            .await?;

        Ok(RollbackResponse {
            balance: user_wallet.amount.to_string(),
        })
    }

    #[tracing::instrument]
    pub async fn update(&self, payload: UpdateRequest) -> Result<UpdateResponse, KgsStatus> {
        let wallet_info = self.mapper.to_wallet_info(&payload).await?;

        let new_amount = payload.get_new_amount()?;
        let old_amount = payload.get_old_amount()?;
        let effective_bet = payload.get_effective_bet()?;
        let rollover_rate = payload.get_rollover_rate()?;

        // 獲取更新策略
        let strategy = UpdateStrategyFactory::new(
            &wallet_info.wallet_source,
            self.wallet_source_repo.clone(),
            self.wallet_service.clone(),
            self.rollover_service.clone(),
        )?;

        // 依照不同的錢包來源執行不同的策略
        strategy
            .apply(
                &wallet_info,
                payload.transaction_id,
                old_amount,
                new_amount,
                effective_bet,
                rollover_rate,
            )
            .await?;

        let user_wallet = self
            .wallet_service
            .get_or_create_new_one(&wallet_info)
            .await?;

        Ok(UpdateResponse {
            balance: user_wallet.amount.to_string(),
        })
    }
}
