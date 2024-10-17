use bigdecimal::{BigDecimal, Zero};
use std::str::FromStr;

use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::warn;

use crate::application::dto::common::*;

impl GetWalletInfoTrait for protos::player_wallet::PlayerWalletRequest {
    fn get_user_id(&self) -> i64 {
        self.player_id
    }
    fn get_client_id(&self) -> i64 {
        self.client_id
    }
    fn get_currency_name(&self) -> &str {
        &self.currency
    }
    fn get_wallet_source_id(&self) -> i64 {
        self.wallet_source_id
    }
}

impl GetWalletInfoTrait for protos::player_wallet::PlayerWalletChangeRequest {
    fn get_user_id(&self) -> i64 {
        self.player_id
    }
    fn get_client_id(&self) -> i64 {
        self.client_id
    }
    fn get_currency_name(&self) -> &str {
        &self.currency
    }
    fn get_wallet_source_id(&self) -> i64 {
        self.wallet_source_id
    }
}

impl GetAmountTrait for protos::player_wallet::PlayerWalletChangeRequest {
    fn get_amount(&self) -> Result<BigDecimal, KgsStatus> {
        // Parse amount
        let amount = BigDecimal::from_str(&self.amount).map_err(|e| {
            warn!("轉換金額失敗: {}", e);
            KgsStatus::InvalidArgument
        })?;

        // Check amount is greater than 0
        if amount <= BigDecimal::zero() {
            warn!("金額 小於等於0");
            return Err(KgsStatus::InvalidArgument);
        }

        Ok(amount)
    }
}

impl GetRolloverRate for protos::player_wallet::PlayerWalletChangeRequest {
    fn get_rollover_rate(&self) -> Result<BigDecimal, KgsStatus> {
        match self.rollover_rate.as_ref() {
            Some(rollover_rate_str) => {
                let rollover_rate = BigDecimal::from_str(rollover_rate_str).map_err(|e| {
                    warn!("轉換有效投注失敗{}", e);
                    KgsStatus::InvalidArgument
                })?;

                if rollover_rate <= BigDecimal::zero() {
                    warn!("流水比率:{} 小於等於0 ", &rollover_rate);
                    return Err(KgsStatus::InvalidArgument);
                }

                Ok(rollover_rate)
            }
            None => Ok(BigDecimal::zero()),
        }
    }
}
