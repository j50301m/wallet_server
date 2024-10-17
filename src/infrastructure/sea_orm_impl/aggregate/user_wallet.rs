use bigdecimal::BigDecimal;
use sea_orm::FromQueryResult;

#[derive(FromQueryResult)]
pub struct UserWalletWithRollover {
    // UserWallet
    pub id: i64,               // UserWallet ID
    pub client_id: i64,        // 客戶ID
    pub user_id: i64,          // 用戶ID
    pub currency_id: i64,      // 幣別ID
    pub currency_name: String, // 貨幣名稱
    pub wallet_source_id: i64, // 錢包來源
    pub amount: BigDecimal,    // 錢包金額
    // WalletSource
    pub wallet_source_name: String, // 錢包來源名稱
    // RolloverMain
    pub requirement_rollover: BigDecimal, // 需求流水
    pub achievement_rollover: BigDecimal, // 達成流水
}

impl Into<crate::domain::UserWalletWithRollover> for UserWalletWithRollover {
    fn into(self) -> crate::domain::UserWalletWithRollover {
        crate::domain::UserWalletWithRollover {
            id: self.id,
            client_id: self.client_id,
            user_id: self.user_id,
            currency_id: self.currency_id,
            currency_name: self.currency_name,
            wallet_source_id: self.wallet_source_id,
            amount: self.amount,
            wallet_source_name: self.wallet_source_name,
            requirement_rollover: self.requirement_rollover,
            achievement_rollover: self.achievement_rollover,
        }
    }
}
