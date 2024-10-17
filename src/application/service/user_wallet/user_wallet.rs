use std::fmt::Debug;
use std::sync::Arc;

use kgs_tracing::tracing;
use protos::player_wallet::*;

use super::rollback_strategy;
use crate::application::dto::{self, *};
use crate::domain;
use crate::enums;
use kgs_err::models::status::Status as KgsStatus;

#[derive(Debug)]
pub struct UserWalletService {
    user_wallet_repo: Arc<dyn domain::UserWalletRepositoryTrait>,
    wallet_source_repo: Arc<dyn domain::WalletSourceRepositoryTrait>,
    wallet_service: Arc<dyn domain::WalletServiceTrait>,
    rollover_service: Arc<dyn domain::RolloverServiceTrait>,
    wallet_mapper: Arc<dyn dto::WalletMapperTrait>,
    query_mapper: Arc<dyn dto::QueryMapperTrait>,
    currency_service: Arc<dyn domain::CurrencyServiceTrait>,
}

impl UserWalletService {
    pub fn new(
        user_wallet_repo: Arc<dyn domain::UserWalletRepositoryTrait>,
        wallet_source_repo: Arc<dyn domain::WalletSourceRepositoryTrait>,
        wallet_service: Arc<dyn domain::WalletServiceTrait>,
        rollover_service: Arc<dyn domain::RolloverServiceTrait>,
        wallet_mapper: Arc<dyn dto::WalletMapperTrait>,
        query_mapper: Arc<dyn dto::QueryMapperTrait>,
        currency_service: Arc<dyn domain::CurrencyServiceTrait>,
    ) -> Self {
        Self {
            user_wallet_repo,
            wallet_source_repo,
            wallet_service,
            rollover_service,
            wallet_mapper,
            query_mapper,
            currency_service,
        }
    }
}

impl UserWalletService {
    #[tracing::instrument]
    #[transactional(SeaOrmPostgres)]
    pub async fn get(&self, payload: PlayerWalletRequest) -> Result<WalletModel, KgsStatus> {
        let wallet_info = self.wallet_mapper.to_wallet_info(&payload).await?;

        let user_wallet = self
            .wallet_service
            .get_or_create_new_one(&wallet_info)
            .await?;

        let rollover_main = self
            .rollover_service
            .get_or_create_new_one(user_wallet.id, &wallet_info)
            .await?;

        self.wallet_mapper
            .to_wallet_proto(user_wallet, rollover_main)
    }

    #[tracing::instrument]
    #[transactional(SeaOrmPostgres)]
    pub async fn get_list(
        &self,
        payload: GetPlayerWalletListRequest,
    ) -> Result<GetPlayerWalletListResponse, KgsStatus> {
        let query = self.query_mapper.to_select_wallet_query(payload).await?;

        let result = self
            .user_wallet_repo
            .get_user_wallets_with_rollover(query)
            .await?;

        Ok(result.to_proto())
    }

    #[tracing::instrument]
    #[transactional(SeaOrmPostgres)]
    pub async fn deposit(
        &self,
        payload: PlayerWalletChangeRequest,
    ) -> Result<WalletModel, KgsStatus> {
        let wallet_info = self.wallet_mapper.to_wallet_info(&payload).await?;
        let amount = payload.get_amount()?;
        let rollover_rate = payload.get_rollover_rate()?;

        // 錢包上分
        let (user_wallet, wallet_txn) = self
            .wallet_service
            .change_amount(
                &wallet_info,
                0,
                payload.wallet_source_id,
                amount.clone(),
                &enums::WalletAction::PaymentDeposit,
            )
            .await?;

        // 修改流水
        let (rollover_main, _rollover_detail) = self
            .rollover_service
            .change_rollover(
                user_wallet.id,
                &wallet_info,
                wallet_txn.id,
                amount,
                rollover_rate,
                enums::WalletAction::PaymentDeposit,
                wallet_info.user_id,
            )
            .await?;

        self.wallet_mapper
            .to_wallet_proto(user_wallet, rollover_main)
    }

    #[tracing::instrument]
    #[transactional(SeaOrmPostgres)]
    pub async fn withdraw(
        &self,
        payload: PlayerWalletChangeRequest,
    ) -> Result<WalletModel, KgsStatus> {
        let wallet_info = self.wallet_mapper.to_wallet_info(&payload).await?;
        let amount = payload.get_amount()?;
        let rollover_rate = payload.get_rollover_rate()?;

        // 確認流水是否達成
        if !self
            .rollover_service
            .is_rollover_achieved(&wallet_info)
            .await?
        {
            return Err(KgsStatus::RolloverNotAchieved);
        }

        // 錢包下分
        let (user_wallet, wallet_txn) = self
            .wallet_service
            .change_amount(
                &wallet_info,
                0,
                payload.wallet_source_id,
                amount.clone(),
                &enums::WalletAction::PaymentWithdraw,
            )
            .await?;

        // 修改流水
        let (rollover_main, _rollover_detail) = self
            .rollover_service
            .change_rollover(
                user_wallet.id,
                &wallet_info,
                wallet_txn.id,
                amount,
                rollover_rate,
                enums::WalletAction::PaymentWithdraw,
                wallet_info.user_id,
            )
            .await?;

        self.wallet_mapper
            .to_wallet_proto(user_wallet, rollover_main)
    }

    #[tracing::instrument]
    pub async fn rollback(&self, payload: RollbackRequest) -> Result<WalletModel, KgsStatus> {
        let wallet_source = self
            .wallet_source_repo
            .get(enums::WalletSource::Normal.to_id())
            .await?;

        let strategy = rollback_strategy::RollbackWalletStrategyFactory::new(
            &wallet_source,
            self.currency_service.clone(),
            self.wallet_source_repo.clone(),
            self.wallet_service.clone(),
            self.rollover_service.clone(),
        )?;

        let (user_wallet, rollover_main) = strategy
            .apply(
                payload.client_id,
                payload.user_id,
                wallet_source,
                payload.source_transaction_id,
            )
            .await?;

        self.wallet_mapper
            .to_wallet_proto(user_wallet, rollover_main)
    }
}
