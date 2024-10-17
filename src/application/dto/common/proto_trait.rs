use std::fmt::Debug;

use bigdecimal::BigDecimal;
use kgs_err::models::status::Status as KgsStatus;

pub trait GetWalletInfoTrait: Sync + Send + Debug {
    fn get_user_id(&self) -> i64;
    fn get_client_id(&self) -> i64;
    fn get_currency_name(&self) -> &str;
    fn get_wallet_source_id(&self) -> i64;
}

pub trait GetAmountTrait: Sync + Send + Debug {
    fn get_amount(&self) -> Result<BigDecimal, KgsStatus>;
}

pub trait GetEffectiveBet: Sync + Send + Debug {
    fn get_effective_bet(&self) -> Result<BigDecimal, KgsStatus>;
}

pub trait GetRolloverRate: Sync + Send + Debug {
    fn get_rollover_rate(&self) -> Result<BigDecimal, KgsStatus>;
}

pub trait GetUpdateAmount: Sync + Send + Debug {
    /// 獲取舊金額 可能有正負號
    fn get_old_amount(&self) -> Result<BigDecimal, KgsStatus>;
    /// 獲取新的金額 可能有正負號
    fn get_new_amount(&self) -> Result<BigDecimal, KgsStatus>;
}
