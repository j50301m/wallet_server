use bigdecimal::BigDecimal;
use kgs_tracing::tracing;
use kgs_tracing::warn;
use std::fmt::Debug;
use std::sync::Arc;

use crate::domain::*;
use crate::enums;
use kgs_err::models::status::Status as KgsStatus;

#[tonic::async_trait]
pub trait WalletServiceTrait: Send + Sync + Debug {
    /// 修改錢包金額 並新增交易紀錄
    /// ### 參數
    /// - `wallet_info`: &WalletInfo - 錢包資訊
    /// - `source_txn_id`: i64 - 來源交易 ID
    /// - `amount`: &BigDecimal - 金額
    /// - `action`: enums::wallet_action::Action - 錢包操作
    /// ### 回傳
    /// - `UserWallet` - 更新後的錢包
    /// - `WalletTransaction` - 更新後的交易紀錄
    async fn change_amount(
        &self,
        wallet_info: &WalletInfo,
        parent_wallet_txn_id: i64,
        source_txn_id: i64,
        amount: BigDecimal,
        action: &enums::WalletAction,
    ) -> Result<(UserWallet, WalletTransaction), KgsStatus>;

    /// 檢查錢包金額是否足夠
    /// ### 參數
    /// - `wallet_info`: &WalletInfo - 錢包資訊
    /// - `withdraw_amount`: &BigDecimal - 提款金額
    /// ### 回傳
    /// - `bool` - 是否足夠
    async fn is_wallet_amount_enough(
        &self,
        wallet_info: &WalletInfo,
        withdraw_amount: &BigDecimal,
    ) -> Result<bool, KgsStatus>;

    /// 依照來源交易 ID 取得最後一筆交易紀錄
    /// ### 參數
    /// - `client_id`: i64 - 用戶 ID
    /// - `source_txn_id`: i64 - 來源交易 ID
    /// ### 回傳
    /// - `WalletTransaction` - 交易紀錄
    async fn get_last_transaction_by_source_id(
        &self,
        client_id: i64,
        user_id: i64,
        source_txn_id: i64,
    ) -> Result<WalletTransaction, KgsStatus>;

    /// 回滾交易 到指定的wallet上
    /// ### 參數
    /// - `wallet_info`: &WalletInfo - 錢包資訊
    /// - `need_rollback_wallet_txn`: i64 - 需要回滾的wallet_txn
    /// ### 回傳
    /// - `UserWallet` - 更新後的錢包
    /// - `WalletTransaction` - 更新後的交易紀錄
    async fn rollback_transaction(
        &self,
        wallet_info: &WalletInfo,
        need_rollback_wallet_txn: &WalletTransaction,
    ) -> Result<(UserWallet, WalletTransaction), KgsStatus>;

    /// 取得或創建新的錢包
    async fn get_or_create_new_one(
        &self,
        wallet_info: &WalletInfo,
    ) -> Result<UserWallet, KgsStatus>;
}

#[derive(Debug)]
pub struct WalletService {
    wallet_txn_repo: Arc<dyn WalletTransactionRepositoryTrait>,
    wallet_repo: Arc<dyn UserWalletRepositoryTrait>,
}

impl WalletService {
    pub fn new(
        wallet_txn_repo: Arc<dyn WalletTransactionRepositoryTrait>,
        wallet_repo: Arc<dyn UserWalletRepositoryTrait>,
    ) -> Self {
        Self {
            wallet_txn_repo,
            wallet_repo,
        }
    }
}

#[tonic::async_trait]
impl WalletServiceTrait for WalletService {
    #[tracing::instrument]
    async fn change_amount(
        &self,
        wallet_info: &WalletInfo,
        parent_wallet_txn_id: i64,
        source_txn_id: i64,
        amount: BigDecimal,
        action: &enums::WalletAction,
    ) -> Result<(UserWallet, WalletTransaction), KgsStatus> {
        // 獲取玩家錢包
        let mut user_wallet = self.get_or_create_new_one(wallet_info).await?;

        // 創建交易紀錄
        let wallet_txn = WalletTransaction::create_before_change(
            &user_wallet,
            parent_wallet_txn_id,
            source_txn_id,
            action,
            &amount,
        )
        .await;

        // 更新錢包
        match action {
            enums::WalletAction::GameDeposit | enums::WalletAction::PaymentDeposit => {
                user_wallet.deposit(&amount);
            }
            enums::WalletAction::GameWithdraw | enums::WalletAction::PaymentWithdraw => {
                user_wallet.withdraw(&amount);
            }
        }

        // 更新db
        let updated_wallet = self.wallet_repo.update(user_wallet).await?;
        let wallet_txn = self.wallet_txn_repo.insert(wallet_txn).await?;

        Ok((updated_wallet, wallet_txn))
    }

    #[tracing::instrument]
    async fn is_wallet_amount_enough(
        &self,
        wallet_info: &WalletInfo,
        withdraw_amount: &BigDecimal,
    ) -> Result<bool, KgsStatus> {
        // 獲取玩家錢包
        let user_wallet = self.get_or_create_new_one(wallet_info).await?;

        Ok(&user_wallet.amount >= withdraw_amount)
    }

    #[tracing::instrument]
    async fn get_last_transaction_by_source_id(
        &self,
        client_id: i64,
        user_id: i64,
        source_txn_id: i64,
    ) -> Result<WalletTransaction, KgsStatus> {
        let wallet_txn_list = self
            .wallet_txn_repo
            .get_list_by_transaction_source_id(client_id, user_id, source_txn_id)
            .await?;

        // 沒有找到交易紀錄 直接返回
        if wallet_txn_list.is_empty() {
            warn!("找不到交易紀錄");
            return Err(KgsStatus::DataNotFound);
        }

        // 定義內部函數 用來排序交易紀錄
        let sort_by_patent_id =
            |mut list: Vec<WalletTransaction>| -> Result<Vec<WalletTransaction>, KgsStatus> {
                // 找到root 並放到最前面
                let root_index = list.iter().position(|x| x.parent_id == 0).ok_or_else(|| {
                    warn!("交易單排序發生錯誤");
                    KgsStatus::InternalServerError
                })?;
                list.swap(0, root_index);

                // 依照parent_id 排序
                let mut sorted_index = 0;
                while sorted_index < list.len() - 1 {
                    for (i, item) in list[sorted_index + 1..].iter().enumerate() {
                        if item.parent_id == list[sorted_index].id {
                            list.swap(sorted_index + 1, sorted_index + 1 + i);
                            sorted_index += 1;
                            break;
                        }
                    }
                }

                Ok(list)
            };

        // 取出最後一筆交易紀錄
        sort_by_patent_id(wallet_txn_list)?.pop().ok_or_else(|| {
            warn!("交易單排序發生錯誤");
            KgsStatus::InternalServerError
        })
    }

    async fn rollback_transaction(
        &self,
        wallet_info: &WalletInfo,
        need_wallet_txn: &WalletTransaction,
    ) -> Result<(UserWallet, WalletTransaction), KgsStatus> {
        // rollback 錢包金額
        let rollback_amount = need_wallet_txn.change_amount.abs();
        let rollback_action = match enums::WalletAction::from_i32(need_wallet_txn.action)? {
            enums::WalletAction::GameDeposit => enums::WalletAction::GameWithdraw,
            enums::WalletAction::GameWithdraw => enums::WalletAction::GameDeposit,
            enums::WalletAction::PaymentDeposit => enums::WalletAction::PaymentWithdraw,
            enums::WalletAction::PaymentWithdraw => enums::WalletAction::PaymentDeposit,
        };

        self.change_amount(
            &wallet_info,
            need_wallet_txn.id,
            need_wallet_txn.transaction_source_id,
            rollback_amount,
            &rollback_action,
        )
        .await
    }

    async fn get_or_create_new_one(
        &self,
        wallet_info: &WalletInfo,
    ) -> Result<UserWallet, KgsStatus> {
        match self.wallet_repo.get(wallet_info).await? {
            Some(wallet) => Ok(wallet),

            None => {
                let new_wallet = UserWallet::new(wallet_info).await;
                self.wallet_repo.insert(new_wallet).await
            }
        }
    }
}
