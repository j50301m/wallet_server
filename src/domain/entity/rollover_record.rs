use bigdecimal::{BigDecimal, One, Zero};

use super::RolloverMain;
use crate::domain::vo;
use crate::enums::RolloverType;
use crate::infrastructure;

#[derive(Debug)]
pub struct RolloverRecord {
    pub id: i64,
    pub main_id: i64, // 對應到流水主表的 ID
    pub client_id: i64,
    pub user_id: i64,
    pub requirement_rollover: BigDecimal, // 需求流水(有正負號)
    pub requirement_rollover_rate: BigDecimal, // 因為流水倍率是可被動的 所以需要紀錄計算當下的流水倍率
    pub achievement_rollover: BigDecimal,      // 達成流水(有正負號)
    pub achievement_rollover_rate: BigDecimal, // 因為流水倍率是可被動的 所以需要紀錄計算當下的流水倍率
    pub create_by: i64,                        // 創建此筆紀錄的user_id
    pub wallet_txn_id: i64,                    // 對應的wallet txn id
    pub create_at: chrono::NaiveDateTime,      // 創建時間
}

impl RolloverRecord {
    /// 創建一筆流水紀錄
    /// ### 參數
    /// - `rollover_main_id`: i64 - 流水主表 ID
    /// - `wallet_txn_id`: i64 - 錢包交易 ID
    /// - `wallet_info`: &vo::WalletInfo - 錢包資訊
    /// - `rollover_type`: RolloverType - 流水類型
    /// - `rollover_amount`: BigDecimal - 流水金額
    /// - `rollover_rate`: BigDecimal - 流水倍率
    /// - `create_by`: i64 - 創建者 ID
    pub async fn new(
        rollover_main_id: i64,
        wallet_txn_id: i64,
        wallet_info: &vo::WalletInfo,
        rollover_type: RolloverType,
        rollover_amount: &BigDecimal,
        rollover_rate: &BigDecimal,
        create_by: i64,
    ) -> RolloverRecord {
        let (achievement_rollover, requirement_rollover) = match rollover_type {
            RolloverType::Requirement => (BigDecimal::zero(), rollover_amount * rollover_rate),
            RolloverType::Achievement => (rollover_amount * rollover_rate, BigDecimal::zero()),
        };

        let (achievement_rollover_rate, requirement_rollover_rate) = match rollover_type {
            RolloverType::Requirement => (BigDecimal::zero(), rollover_rate.clone()),
            RolloverType::Achievement => (rollover_rate.clone(), BigDecimal::zero()),
        };

        RolloverRecord {
            id: infrastructure::snowflake::generate_id().await,
            main_id: rollover_main_id,
            client_id: wallet_info.client_id,
            user_id: wallet_info.user_id,
            requirement_rollover,
            requirement_rollover_rate,
            achievement_rollover,
            achievement_rollover_rate,
            create_by,
            wallet_txn_id,
            create_at: chrono::Utc::now().naive_utc(),
        }
    }

    /// 創建一筆清除流水紀錄
    pub async fn create_clear_rollover_record(
        origin_rollover_main: &RolloverMain,
        wallet_txn_id: i64,
        create_by: i64,
    ) -> RolloverRecord {
        let requirement_rollover = -origin_rollover_main.requirement_rollover.clone();
        let achievement_rollover = -origin_rollover_main.achievement_rollover.clone();

        RolloverRecord {
            id: infrastructure::snowflake::generate_id().await,
            main_id: origin_rollover_main.id,
            client_id: origin_rollover_main.client_id,
            user_id: origin_rollover_main.user_id,
            requirement_rollover,
            requirement_rollover_rate: BigDecimal::one(),
            achievement_rollover,
            achievement_rollover_rate: BigDecimal::one(),
            create_by,
            wallet_txn_id,
            create_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub async fn create_rollback_rollover_record(
        self,
        wallet_txn_id: i64,
        create_by: i64,
    ) -> RolloverRecord {
        let requirement_rollover = -self.requirement_rollover;
        let achievement_rollover = -self.achievement_rollover;

        RolloverRecord {
            id: infrastructure::snowflake::generate_id().await,
            main_id: self.main_id,
            client_id: self.client_id,
            user_id: self.user_id,
            requirement_rollover,
            requirement_rollover_rate: self.requirement_rollover_rate,
            achievement_rollover,
            achievement_rollover_rate: self.achievement_rollover_rate,
            create_by,
            wallet_txn_id,
            create_at: chrono::Utc::now().naive_utc(),
        }
    }
}
