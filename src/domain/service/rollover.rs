use std::fmt::Debug;
use std::sync::Arc;

use bigdecimal::BigDecimal;
use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::tracing;

use crate::domain::*;
use crate::enums;
use crate::enums::RolloverType;

#[tonic::async_trait]
pub trait RolloverServiceTrait: Send + Sync + Debug {
    /// 修改流水主表 並新增流水紀錄
    /// ### 參數
    /// - `wallet_info`: &WalletInfo - 錢包資訊
    /// - `wallet_txn_id`: i64 - 錢包交易 ID
    /// - `amount`: BigDecimal - 流水金額
    /// - `rollover_rate`: BigDecimal - 流水倍率
    /// - `action`: enums::wallet_action::Action - 錢包操作
    /// ### 回傳
    /// - `RolloverMain` - 流水主表
    /// - `Option<RolloverRecord>` - 流水紀錄
    async fn change_rollover(
        &self,
        user_wallet_id: i64,
        wallet_info: &WalletInfo,
        wallet_txn_id: i64,
        amount: BigDecimal,
        rollover_rate: BigDecimal,
        action: enums::WalletAction,
        change_by: i64,
    ) -> Result<(RolloverMain, Option<RolloverRecord>), KgsStatus>;

    /// Rollback 指定wallet_txn的流水
    async fn rollback_rollover(
        &self,
        user_wallet_id: i64,
        wallet_info: &WalletInfo,
        origin_wallet_txn_id: i64,
        rollback_wallet_txn_id: i64,
        create_by: i64,
    ) -> Result<(RolloverMain, Option<RolloverRecord>), KgsStatus>;

    /// 取得或創建新的流水主表
    async fn get_or_create_new_one(
        &self,
        user_wallet_id: i64,
        wallet_info: &WalletInfo,
    ) -> Result<RolloverMain, KgsStatus>;

    /// 判斷流水是否達成
    async fn is_rollover_achieved(&self, wallet_info: &WalletInfo) -> Result<bool, KgsStatus>;
}

#[derive(Debug)]
pub struct RolloverService {
    main_rollover_repo: Arc<dyn RolloverMainRepositoryTrait>,
    rollover_record_repo: Arc<dyn RolloverRecordRepositoryTrait>,
}

impl RolloverService {
    pub fn new(
        main_rollover_repo: Arc<dyn RolloverMainRepositoryTrait>,
        rollover_record_repo: Arc<dyn RolloverRecordRepositoryTrait>,
    ) -> Self {
        Self {
            main_rollover_repo,
            rollover_record_repo,
        }
    }
}

#[tonic::async_trait]
impl RolloverServiceTrait for RolloverService {
    #[tracing::instrument]
    async fn change_rollover(
        &self,
        user_wallet_id: i64,
        wallet_info: &WalletInfo,
        wallet_txn_id: i64,
        amount: BigDecimal,
        rollover_rate: BigDecimal,
        action: enums::WalletAction,
        change_by: i64,
    ) -> Result<(RolloverMain, Option<RolloverRecord>), KgsStatus> {
        match action {
            enums::WalletAction::GameDeposit => {
                self.game_deposit_rollover(
                    user_wallet_id,
                    wallet_info,
                    wallet_txn_id,
                    &amount,
                    &rollover_rate,
                    change_by,
                )
                .await
            }
            enums::WalletAction::GameWithdraw => {
                self.game_withdraw_rollover(user_wallet_id, wallet_info)
                    .await
            }
            enums::WalletAction::PaymentDeposit => {
                self.payment_deposit_rollover(
                    user_wallet_id,
                    wallet_info,
                    wallet_txn_id,
                    &amount,
                    &rollover_rate,
                    change_by,
                )
                .await
            }
            enums::WalletAction::PaymentWithdraw => {
                self.payment_withdraw_rollover(
                    user_wallet_id,
                    wallet_info,
                    wallet_txn_id,
                    change_by,
                )
                .await
            }
        }
    }

    #[tracing::instrument]
    async fn rollback_rollover(
        &self,
        user_wallet_id: i64,
        wallet_info: &WalletInfo,
        origin_wallet_txn_id: i64,
        rollback_wallet_txn_id: i64,
        create_by: i64,
    ) -> Result<(RolloverMain, Option<RolloverRecord>), KgsStatus> {
        let rollover_record = self
            .rollover_record_repo
            .get_opt_by_wallet_transaction_id(origin_wallet_txn_id)
            .await?;
        if let Some(record) = rollover_record {
            // 創建rollback紀錄
            let rollback_record = record
                .create_rollback_rollover_record(rollback_wallet_txn_id, create_by)
                .await;

            // 插入流水紀錄
            let rollback_record = self.rollover_record_repo.insert(rollback_record).await?;

            // 更新主表
            let rollover_main = self
                .update_rollover_main_by_wallet_info(
                    user_wallet_id,
                    wallet_info,
                    &rollback_record.requirement_rollover,
                    &rollback_record.achievement_rollover,
                )
                .await?;

            return Ok((rollover_main, Some(rollback_record)));
        };

        let rollover_main = self
            .get_or_create_new_one(user_wallet_id, wallet_info)
            .await?;
        Ok((rollover_main, None))
    }

    async fn get_or_create_new_one(
        &self,
        user_wallet_id: i64,
        wallet_info: &WalletInfo,
    ) -> Result<RolloverMain, KgsStatus> {
        let rollover = self.main_rollover_repo.get(wallet_info).await?;
        match rollover {
            Some(rollover) => Ok(rollover),
            None => {
                let rollover = RolloverMain::new(wallet_info, user_wallet_id).await;
                self.main_rollover_repo.insert(rollover).await
            }
        }
    }

    async fn is_rollover_achieved(&self, wallet_info: &WalletInfo) -> Result<bool, KgsStatus> {
        let rollover_main = self
            .main_rollover_repo
            .get(wallet_info)
            .await?
            .ok_or_else(|| {
                tracing::error!("cnanot find rollover main");
                KgsStatus::DataNotFound
            })?;

        Ok(rollover_main.achievement_rollover >= rollover_main.requirement_rollover)
    }
}

impl RolloverService {
    /// 遊戲上分時增加有達成流水
    #[tracing::instrument]
    async fn game_deposit_rollover(
        &self,
        user_wallet_id: i64,
        wallet_info: &WalletInfo,
        wallet_txn_id: i64,
        rollover_amount: &BigDecimal,
        rollover_rate: &BigDecimal,
        change_by: i64,
    ) -> Result<(RolloverMain, Option<RolloverRecord>), KgsStatus> {
        // 獲取流水主表
        let mut rollover_main = self
            .get_or_create_new_one(user_wallet_id, wallet_info)
            .await?;

        // 創建流水紀錄
        let rollover_record = RolloverRecord::new(
            rollover_main.id,
            wallet_txn_id,
            &wallet_info,
            RolloverType::Achievement,
            rollover_amount,
            rollover_rate,
            change_by,
        )
        .await;

        // 主表增加流水
        rollover_main.add_achievement_rollover_by_amount(rollover_amount, rollover_rate);

        // 保存至db
        let rollover_record = self.rollover_record_repo.insert(rollover_record).await?;
        let rollover_main = self.main_rollover_repo.update(rollover_main).await?;

        Ok((rollover_main, Some(rollover_record)))
    }

    /// 遊戲下注時不需要修改流水
    #[tracing::instrument]
    async fn game_withdraw_rollover(
        &self,
        user_wallet_id: i64,
        wallet_info: &WalletInfo,
    ) -> Result<(RolloverMain, Option<RolloverRecord>), KgsStatus> {
        // 獲取流水主表
        let rollover_main = self
            .get_or_create_new_one(user_wallet_id, wallet_info)
            .await?;

        Ok((rollover_main, None))
    }

    /// 入金時增加需求流水
    #[tracing::instrument]
    async fn payment_deposit_rollover(
        &self,
        user_wallet_id: i64,
        wallet_info: &WalletInfo,
        wallet_txn_id: i64,
        rollover_amount: &BigDecimal,
        rollover_rate: &BigDecimal,
        change_by: i64,
    ) -> Result<(RolloverMain, Option<RolloverRecord>), KgsStatus> {
        // 獲取流水主表
        let mut rollover_main = self
            .get_or_create_new_one(user_wallet_id, wallet_info)
            .await?;

        // 創建流水紀錄
        let rollover_record = RolloverRecord::new(
            rollover_main.id,
            wallet_txn_id,
            &wallet_info,
            RolloverType::Requirement,
            rollover_amount,
            rollover_rate,
            change_by,
        )
        .await;

        // 主表增加流水
        rollover_main.add_requirement_rollover_by_amount(rollover_amount, rollover_rate);

        // 保存至db
        let rollover_record = self.rollover_record_repo.insert(rollover_record).await?;
        let rollover_main = self.main_rollover_repo.update(rollover_main).await?;

        Ok((rollover_main, Some(rollover_record)))
    }

    /// 出金時需求流水與達成流水清零
    #[tracing::instrument]
    async fn payment_withdraw_rollover(
        &self,
        user_wallet_id: i64,
        wallet_info: &WalletInfo,
        wallet_txn_id: i64,
        change_by: i64,
    ) -> Result<(RolloverMain, Option<RolloverRecord>), KgsStatus> {
        // 獲取流水主表
        let mut rollover_main = self
            .get_or_create_new_one(wallet_txn_id, wallet_info)
            .await?;

        // 新增清零流水紀錄
        let rollover_record =
            RolloverRecord::create_clear_rollover_record(&rollover_main, wallet_txn_id, change_by)
                .await;

        // 主表流水清空
        rollover_main.clear_rollover();

        // 保存至db
        let rollover_record = self.rollover_record_repo.insert(rollover_record).await?;
        let rollover_main = self.main_rollover_repo.update(rollover_main).await?;

        Ok((rollover_main, Some(rollover_record)))
    }

    #[tracing::instrument]
    async fn update_rollover_main_by_wallet_info(
        &self,
        user_wallet_id: i64,
        wallet_info: &WalletInfo,
        requirement_rollover: &BigDecimal,
        achievement_rollover: &BigDecimal,
    ) -> Result<RolloverMain, KgsStatus> {
        let mut rollover_main = self
            .get_or_create_new_one(user_wallet_id, wallet_info)
            .await?;

        rollover_main.add_achievement_rollover(achievement_rollover);
        rollover_main.add_requirement_rollover(requirement_rollover);

        self.main_rollover_repo.update(rollover_main).await
    }
}
