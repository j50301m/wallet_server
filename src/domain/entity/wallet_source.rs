#[derive(Debug)]
pub struct WalletSource {
    pub id: i64,
    pub name: String,
    pub create_at: chrono::NaiveDateTime,
}
