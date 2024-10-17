// use kgs_err::models::status::Status as KgsStatus;

#[derive(Debug, PartialEq)]
pub enum CurrencyStatus {
    // Disable = 0,
    Enable = 1,
}

impl CurrencyStatus {
    pub fn to_id(&self) -> i32 {
        match self {
            CurrencyStatus::Disable => 0,
            CurrencyStatus::Enable => 1,
        }
    }

    pub fn from_id(id: i32) -> Result<CurrencyStatus, KgsStatus> {
        match id {
            0 => Ok(CurrencyStatus::Disable),
            1 => Ok(CurrencyStatus::Enable),
            _ => Err(KgsStatus::InvalidArgument),
        }
    }
}
