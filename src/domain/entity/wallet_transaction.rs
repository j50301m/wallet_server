use crate::domain;
use crate::enums;
use crate::infrastructure;
use bigdecimal::BigDecimal;

#[derive(Debug)]
pub struct WalletTransaction {
    pub id: i64,
    pub parent_id: i64, // 關聯單的id
    pub client_id: i64,
    pub user_id: i64,
    pub currency_id: i64,
    pub wallet_source_id: i64,
    pub action: i32,
    pub transaction_source_id: i64,
    pub before_amount: BigDecimal,
    pub change_amount: BigDecimal,
    pub after_amount: BigDecimal,
    pub status: i32,
    pub create_at: chrono::NaiveDateTime,
    pub update_at: chrono::NaiveDateTime,
}

impl WalletTransaction {
    /// ### 創建交易紀錄 在錢包未修改之前
    pub async fn create_before_change(
        before_changed_wallet: &domain::UserWallet,
        parent_txn_id: i64,
        source_txn_id: i64,
        action: &enums::WalletAction,
        change_amount: &BigDecimal,
    ) -> WalletTransaction {
        let before_amount = before_changed_wallet.amount.clone();
        let after_amount = match action {
            enums::WalletAction::PaymentDeposit | enums::WalletAction::GameDeposit => {
                &before_amount + change_amount
            }
            enums::WalletAction::PaymentWithdraw | enums::WalletAction::GameWithdraw => {
                &before_amount - change_amount
            }
        };
        let now = chrono::Utc::now().naive_utc();

        WalletTransaction {
            id: infrastructure::snowflake::generate_id().await,
            parent_id: parent_txn_id,
            client_id: before_changed_wallet.client_id,
            user_id: before_changed_wallet.user_id,
            currency_id: before_changed_wallet.currency_id,
            wallet_source_id: before_changed_wallet.wallet_source_id,
            action: action.to_id(),
            transaction_source_id: source_txn_id,
            before_amount,
            change_amount: change_amount.clone(),
            after_amount,
            status: enums::WalletStatus::Success.to_id(),
            create_at: now,
            update_at: now,
        }
    }

    /// ### 創建交易紀錄 在錢包修改之後
    pub async fn create_after_change(
        after_changed_wallet: &domain::UserWallet,
        parent_txn_id: i64,
        source_txn_id: i64,
        action: &enums::WalletAction,
        change_amount: &BigDecimal,
    ) -> WalletTransaction {
        let after_amount = after_changed_wallet.amount.clone();
        let before_amount = match action {
            enums::WalletAction::PaymentDeposit | enums::WalletAction::GameDeposit => {
                &after_amount - change_amount
            }
            enums::WalletAction::PaymentWithdraw | enums::WalletAction::GameWithdraw => {
                &after_amount + change_amount
            }
        };
        let now = chrono::Utc::now().naive_utc();

        WalletTransaction {
            id: infrastructure::snowflake::generate_id().await,
            parent_id: parent_txn_id,
            client_id: after_changed_wallet.client_id,
            user_id: after_changed_wallet.user_id,
            currency_id: after_changed_wallet.currency_id,
            wallet_source_id: after_changed_wallet.wallet_source_id,
            action: action.to_id(),
            transaction_source_id: source_txn_id,
            before_amount,
            change_amount: change_amount.clone(),
            after_amount,
            status: enums::WalletStatus::Success.to_id(),
            create_at: now,
            update_at: now,
        }
    }
}
