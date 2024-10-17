use sea_orm::{entity::prelude::*, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "rollover_record")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
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

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    RolloverMain,
    WalletTransaction,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::RolloverMain => Entity::belongs_to(super::rollover_main::Entity)
                .from(Column::MainId)
                .to(super::rollover_main::Column::Id)
                .into(),
            Self::WalletTransaction => Entity::belongs_to(super::wallet_transaction::Entity)
                .from(Column::WalletTxnId)
                .to(super::wallet_transaction::Column::Id)
                .into(),
        }
    }
}

impl Related<super::rollover_main::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolloverMain.def()
    }
}

impl Related<super::wallet_transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WalletTransaction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Into<crate::domain::RolloverRecord> for Model {
    fn into(self) -> crate::domain::RolloverRecord {
        crate::domain::RolloverRecord {
            id: self.id,
            main_id: self.main_id,
            client_id: self.client_id,
            user_id: self.user_id,
            requirement_rollover: self.requirement_rollover,
            requirement_rollover_rate: self.requirement_rollover_rate,
            achievement_rollover: self.achievement_rollover,
            achievement_rollover_rate: self.achievement_rollover_rate,
            create_by: self.create_by,
            wallet_txn_id: self.wallet_txn_id,
            create_at: self.create_at,
        }
    }
}

impl From<crate::domain::RolloverRecord> for ActiveModel {
    fn from(value: crate::domain::RolloverRecord) -> Self {
        Self {
            id: Set(value.id),
            main_id: Set(value.main_id),
            client_id: Set(value.client_id),
            user_id: Set(value.user_id),
            requirement_rollover: Set(value.requirement_rollover),
            requirement_rollover_rate: Set(value.requirement_rollover_rate),
            achievement_rollover: Set(value.achievement_rollover),
            achievement_rollover_rate: Set(value.achievement_rollover_rate),
            create_by: Set(value.create_by),
            wallet_txn_id: Set(value.wallet_txn_id),
            create_at: Set(value.create_at),
        }
    }
}
