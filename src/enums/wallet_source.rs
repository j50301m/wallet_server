use derive_more::Display;
use kgs_err::models::status::Status as KgsStatus;

#[derive(Debug, Copy, Clone, Display, PartialEq)]
pub enum WalletSource {
    #[display(fmt = "normal_wallet")]
    Normal = 1,
    #[display(fmt = "bonus_wallet")]
    Bonus = 2,
}

impl WalletSource {
    pub fn from_id(wallet_source_id: i64) -> Result<WalletSource, KgsStatus> {
        match wallet_source_id {
            1 => Ok(WalletSource::Normal),
            2 => Ok(WalletSource::Bonus),
            _ => Err(KgsStatus::InvalidArgument),
        }
    }

    pub fn to_id(&self) -> i64 {
        match self {
            WalletSource::Normal => 1,
            WalletSource::Bonus => 2,
        }
    }
}
