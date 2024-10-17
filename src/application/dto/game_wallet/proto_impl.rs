use std::str::FromStr;

use bigdecimal::{BigDecimal, Zero};
use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::warn;

use crate::application::dto::common::*;

impl GetWalletInfoTrait for protos::game_wallet::BalanceRequest {
    fn get_user_id(&self) -> i64 {
        self.user_id
    }
    fn get_client_id(&self) -> i64 {
        self.client_id
    }
    fn get_currency_name(&self) -> &str {
        &self.currency
    }
    fn get_wallet_source_id(&self) -> i64 {
        self.wallet_source
    }
}

impl GetWalletInfoTrait for protos::game_wallet::DepositRequest {
    fn get_user_id(&self) -> i64 {
        self.user_id
    }
    fn get_client_id(&self) -> i64 {
        self.client_id
    }
    fn get_currency_name(&self) -> &str {
        &self.currency
    }
    fn get_wallet_source_id(&self) -> i64 {
        self.wallet_source
    }
}

impl GetAmountTrait for protos::game_wallet::DepositRequest {
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

impl GetEffectiveBet for protos::game_wallet::DepositRequest {
    fn get_effective_bet(&self) -> Result<BigDecimal, KgsStatus> {
        let effective_bet = BigDecimal::from_str(&self.effective_bet).map_err(|e| {
            warn!("轉換有效投注失敗: {}", e);
            KgsStatus::InvalidArgument
        })?;

        if effective_bet <= BigDecimal::zero() {
            warn!("有效投注 小於等於0");
            return Err(KgsStatus::InvalidArgument);
        }

        Ok(effective_bet)
    }
}

impl GetRolloverRate for protos::game_wallet::DepositRequest {
    fn get_rollover_rate(&self) -> Result<BigDecimal, KgsStatus> {
        let rollover_rate = BigDecimal::from_str(&self.rollover_rate).map_err(|e| {
            warn!("轉換流水倍率失敗: {}", e);
            KgsStatus::InvalidArgument
        })?;

        if rollover_rate <= BigDecimal::zero() {
            warn!("流水倍率 小於等於0");
            return Err(KgsStatus::InvalidArgument);
        }

        Ok(rollover_rate)
    }
}

impl GetWalletInfoTrait for protos::game_wallet::WithdrawRequest {
    fn get_user_id(&self) -> i64 {
        self.user_id
    }
    fn get_client_id(&self) -> i64 {
        self.client_id
    }
    fn get_currency_name(&self) -> &str {
        &self.currency
    }
    fn get_wallet_source_id(&self) -> i64 {
        self.wallet_source
    }
}

impl GetAmountTrait for protos::game_wallet::WithdrawRequest {
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

impl GetWalletInfoTrait for protos::game_wallet::RollbackRequest {
    fn get_user_id(&self) -> i64 {
        self.user_id
    }
    fn get_client_id(&self) -> i64 {
        self.client_id
    }
    fn get_currency_name(&self) -> &str {
        &self.currency
    }
    fn get_wallet_source_id(&self) -> i64 {
        self.wallet_source
    }
}

impl GetWalletInfoTrait for protos::game_wallet::UpdateRequest {
    fn get_user_id(&self) -> i64 {
        self.user_id
    }
    fn get_client_id(&self) -> i64 {
        self.client_id
    }
    fn get_currency_name(&self) -> &str {
        &self.currency
    }
    fn get_wallet_source_id(&self) -> i64 {
        self.wallet_source
    }
}

impl GetEffectiveBet for protos::game_wallet::UpdateRequest {
    fn get_effective_bet(&self) -> Result<BigDecimal, KgsStatus> {
        let effective_bet = BigDecimal::from_str(&self.effective_bet).map_err(|e| {
            warn!("轉換有效投注失敗: {}", e);
            KgsStatus::InvalidArgument
        })?;

        if effective_bet <= BigDecimal::zero() {
            warn!("有效投注 小於等於0");
            return Err(KgsStatus::InvalidArgument);
        }

        Ok(effective_bet)
    }
}

impl GetUpdateAmount for protos::game_wallet::UpdateRequest {
    fn get_new_amount(&self) -> Result<BigDecimal, KgsStatus> {
        let new_amount = BigDecimal::from_str(&self.new_amount).map_err(|e| {
            warn!("轉換新金額失敗: {}", e);
            KgsStatus::InvalidArgument
        })?;

        Ok(new_amount)
    }

    fn get_old_amount(&self) -> Result<BigDecimal, KgsStatus> {
        let old_amount = BigDecimal::from_str(&self.old_amount).map_err(|e| {
            warn!("轉換舊金額失敗: {}", e);
            KgsStatus::InvalidArgument
        })?;

        Ok(old_amount)
    }
}

impl GetRolloverRate for protos::game_wallet::UpdateRequest {
    fn get_rollover_rate(&self) -> Result<BigDecimal, KgsStatus> {
        let rollover_rate = BigDecimal::from_str(&self.rollover_rate).map_err(|e| {
            warn!("轉換流水倍率失敗: {}", e);
            KgsStatus::InvalidArgument
        })?;

        if rollover_rate <= BigDecimal::zero() {
            warn!("流水倍率 小於等於0");
            return Err(KgsStatus::InvalidArgument);
        }

        Ok(rollover_rate)
    }
}
