use bigdecimal::{BigDecimal, Zero};

use crate::{domain, infrastructure};

#[derive(Debug)]
pub struct RolloverMain {
    pub id: i64,
    pub user_wallet_id: i64,
    pub client_id: i64,
    pub user_id: i64,
    pub currency_id: i64,                 // 幣別id
    pub currency_name: String,            // 幣別名稱
    pub wallet_source_id: i64,            // 錢包來源id
    pub requirement_rollover: BigDecimal, // 需求流水(有正負號)
    pub achievement_rollover: BigDecimal, // 達成流水(有正負號)
    pub create_at: chrono::NaiveDateTime,
    pub update_at: chrono::NaiveDateTime,
}

impl RolloverMain {
    pub async fn new(wallet_info: &domain::WalletInfo, user_wallet_id: i64) -> Self {
        Self {
            id: infrastructure::snowflake::generate_id().await,
            user_wallet_id: user_wallet_id,
            client_id: wallet_info.client_id,
            user_id: wallet_info.user_id,
            currency_id: wallet_info.currency.id,
            currency_name: wallet_info.currency.name.clone(),
            wallet_source_id: wallet_info.wallet_source.id,
            requirement_rollover: BigDecimal::zero(),
            achievement_rollover: BigDecimal::zero(),
            create_at: chrono::Utc::now().naive_utc(),
            update_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn add_requirement_rollover(&mut self, add_requirement_rollover: &BigDecimal) {
        self.requirement_rollover += add_requirement_rollover;
        self.update_at = chrono::Utc::now().naive_utc();
    }

    pub fn add_achievement_rollover(&mut self, add_achievement_rollover: &BigDecimal) {
        self.achievement_rollover += add_achievement_rollover;
        self.update_at = chrono::Utc::now().naive_utc();
    }

    pub fn add_requirement_rollover_by_amount(
        &mut self,
        amount: &BigDecimal,
        rollover_rate: &BigDecimal,
    ) {
        self.requirement_rollover += amount * rollover_rate;
        self.update_at = chrono::Utc::now().naive_utc();
    }

    pub fn add_achievement_rollover_by_amount(
        &mut self,
        amount: &BigDecimal,
        rollover_rate: &BigDecimal,
    ) {
        self.achievement_rollover += amount * rollover_rate;
        self.update_at = chrono::Utc::now().naive_utc();
    }

    pub fn clear_rollover(&mut self) {
        self.requirement_rollover = BigDecimal::zero();
        self.achievement_rollover = BigDecimal::zero();
        self.update_at = chrono::Utc::now().naive_utc();
    }
}
