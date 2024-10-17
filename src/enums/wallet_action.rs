use kgs_err::models::status::Status as KgsStatus;

#[derive(Debug, PartialEq)]
pub enum WalletAction {
    GameDeposit = 1,
    GameWithdraw = 2,
    PaymentDeposit = 3,
    PaymentWithdraw = 4,
    PaymentWithdrawReject = 5,
}

impl WalletAction {
    pub fn from_i32(value: i32) -> Result<WalletAction, KgsStatus> {
        match value {
            1 => Ok(WalletAction::GameDeposit),
            2 => Ok(WalletAction::GameWithdraw),
            3 => Ok(WalletAction::PaymentDeposit),
            4 => Ok(WalletAction::PaymentWithdraw),
            5 => Ok(Action::PaymentWithdrawReject),
            _ => Err(KgsStatus::InvalidArgument),
        }
    }

    pub fn to_id(&self) -> i32 {
        match self {
            WalletAction::GameDeposit => 1,
            WalletAction::GameWithdraw => 2,
            WalletAction::PaymentDeposit => 3,
            WalletAction::PaymentWithdraw => 4,
            Action::PaymentWithdrawReject => 5,
        }
    }
}
