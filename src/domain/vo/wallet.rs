use crate::domain::WalletSource;

/// 錢包基本資訊
#[derive(Debug)]
pub struct WalletInfo {
    pub client_id: i64,
    pub user_id: i64,
    pub currency: Currency,
    pub wallet_source: WalletSource,
}

#[derive(Debug, Clone)]
pub struct Currency {
    pub id: i64,
    pub name: String,
}

#[derive(Debug)]
pub struct SelectWalletsQuery {
    pub client_id: i64,
    pub player_ids: Vec<i64>,
    pub currency_ids: Vec<i64>,
    pub wallet_source_ids: Vec<i64>,
    pub page: u64,
    pub page_size: u64,
}
