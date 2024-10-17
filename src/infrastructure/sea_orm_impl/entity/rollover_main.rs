use sea_orm::{entity::prelude::*, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "rollover_main")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub user_wallet_id: i64, // 對應到user_wallet的 ID
    pub client_id: i64,
    pub user_id: i64,
    pub currency_id: i64,                                   // 幣別id
    pub currency_name: String,                              // 幣別名稱
    pub wallet_source_id: i64,                              // 錢包來源id
    pub requirement_rollover: sea_orm::prelude::BigDecimal, // 需求流水(有正負號)
    pub achievement_rollover: sea_orm::prelude::BigDecimal, // 達成流水(有正負號)
    pub create_at: chrono::NaiveDateTime,
    pub update_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    WalletSource,
    RolloverRecord,
    UserWallet,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::WalletSource => Entity::belongs_to(super::wallet_source::Entity)
                .from(Column::WalletSourceId)
                .to(super::wallet_source::Column::Id)
                .into(),
            Self::RolloverRecord => Entity::has_many(super::rollover_record::Entity).into(),
            Self::UserWallet => Entity::belongs_to(super::user_wallet::Entity)
                .from(Column::UserWalletId)
                .to(super::user_wallet::Column::Id)
                .into(),
        }
    }
}

impl Related<super::wallet_source::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WalletSource.def()
    }
}

impl Related<super::rollover_record::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolloverRecord.def()
    }
}

impl Related<super::user_wallet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserWallet.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Into<crate::domain::RolloverMain> for Model {
    fn into(self) -> crate::domain::RolloverMain {
        crate::domain::RolloverMain {
            id: self.id,
            user_wallet_id: self.user_wallet_id,
            client_id: self.client_id,
            user_id: self.user_id,
            currency_id: self.currency_id,
            currency_name: self.currency_name,
            wallet_source_id: self.wallet_source_id,
            requirement_rollover: self.requirement_rollover,
            achievement_rollover: self.achievement_rollover,
            create_at: self.create_at,
            update_at: self.update_at,
        }
    }
}

impl From<crate::domain::RolloverMain> for ActiveModel {
    fn from(domain: crate::domain::RolloverMain) -> Self {
        Self {
            id: Set(domain.id),
            user_wallet_id: Set(domain.user_wallet_id),
            client_id: Set(domain.client_id),
            user_id: Set(domain.user_id),
            currency_id: Set(domain.currency_id),
            currency_name: Set(domain.currency_name),
            wallet_source_id: Set(domain.wallet_source_id),
            requirement_rollover: Set(domain.requirement_rollover),
            achievement_rollover: Set(domain.achievement_rollover),
            create_at: Set(domain.create_at),
            update_at: Set(domain.update_at),
        }
    }
}
