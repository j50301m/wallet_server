use sea_orm::{entity::prelude::*, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wallet_source")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub create_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::rollover_main::Entity")]
    RolloverMain,
    #[sea_orm(has_many = "super::user_wallet::Entity")]
    UserWallet,
    #[sea_orm(has_many = "super::wallet_transaction::Entity")]
    WalletTransaction,
}

impl Related<super::rollover_main::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolloverMain.def()
    }
}

impl Related<super::user_wallet::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserWallet.def()
    }
}

impl Related<super::wallet_transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WalletTransaction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Into<crate::domain::WalletSource> for Model {
    fn into(self) -> crate::domain::WalletSource {
        crate::domain::WalletSource {
            id: self.id,
            name: self.name,
            create_at: self.create_at,
        }
    }
}

impl From<crate::domain::WalletSource> for ActiveModel {
    fn from(w: crate::domain::WalletSource) -> Self {
        Self {
            id: Set(w.id),
            name: Set(w.name),
            create_at: Set(w.create_at),
        }
    }
}
