pub enum WalletStatus {
    // Pending = 0,
    Success = 1,
    // Cancel = 2,
}

impl WalletStatus {
    pub fn from_i32(value: i32) -> Result<WalletStatus, KgsStatus> {
        match value {
            0 => Ok(WalletStatus::Pending),
            1 => Ok(WalletStatus::Success),
            2 => Ok(WalletStatus::Cancel),
            _ => Err(KgsStatus::InvalidArgument),
        }
    }

    pub fn to_id(&self) -> i32 {
        match self {
            WalletStatus::Pending => 0,
            WalletStatus::Success => 1,
            WalletStatus::Cancel => 2,
        }
    }
}
