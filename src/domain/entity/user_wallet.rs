use bigdecimal::BigDecimal;
use bigdecimal::Zero;

use crate::domain;
use crate::infrastructure;

#[derive(Debug)]
pub struct UserWallet {
    pub id: i64,
    pub client_id: i64,
    pub user_id: i64,
    pub currency_id: i64,
    pub currency_name: String,
    pub wallet_source_id: i64,
    pub wallet_source_name: String,
    pub amount: BigDecimal,
}

impl UserWallet {
    pub async fn new(wallet_info: &domain::WalletInfo) -> Self {
        Self {
            id: infrastructure::snowflake::generate_id().await,
            client_id: wallet_info.client_id,
            user_id: wallet_info.user_id,
            currency_id: wallet_info.currency.id,
            currency_name: wallet_info.currency.name.clone(),
            wallet_source_id: wallet_info.wallet_source.id,
            wallet_source_name: wallet_info.wallet_source.name.clone(),
            amount: BigDecimal::zero(),
        }
    }

    pub fn deposit(&mut self, amount: &BigDecimal) {
        self.amount += amount;
    }

    pub fn withdraw(&mut self, amount: &BigDecimal) {
        self.amount -= amount;
    }
}
