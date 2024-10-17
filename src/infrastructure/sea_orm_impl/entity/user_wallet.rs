use sea_orm::{entity::prelude::*, ActiveValue::NotSet, Set};

use super::rollover_main;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_wallet")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub client_id: i64,
    pub user_id: i64,
    pub currency_id: i64,
    pub currency_name: String,
    pub wallet_source_id: i64,
    pub wallet_source_name: String,
    pub amount: BigDecimal,
    pub create_at: chrono::NaiveDateTime,
    pub update_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    WalletSource,
    RolloverMain,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::WalletSource => Entity::belongs_to(super::wallet_source::Entity)
                .from(Column::WalletSourceId)
                .to(super::wallet_source::Column::Id)
                .into(),
            Self::RolloverMain => Entity::has_one(rollover_main::Entity)
                .from(Column::Id)
                .to(rollover_main::Column::UserWalletId)
                .into(),
        }
    }
}

impl Related<super::wallet_source::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WalletSource.def()
    }
}

impl Related<rollover_main::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolloverMain.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Into<crate::domain::UserWallet> for Model {
    fn into(self) -> crate::domain::UserWallet {
        crate::domain::UserWallet {
            id: self.id,
            client_id: self.client_id,
            user_id: self.user_id,
            currency_id: self.currency_id,
            currency_name: self.currency_name,
            wallet_source_id: self.wallet_source_id,
            wallet_source_name: self.wallet_source_name,
            amount: self.amount,
        }
    }
}

impl From<crate::domain::UserWallet> for ActiveModel {
    fn from(user_wallet: crate::domain::UserWallet) -> Self {
        Self {
            id: Set(user_wallet.id),
            client_id: Set(user_wallet.client_id),
            user_id: Set(user_wallet.user_id),
            currency_id: Set(user_wallet.currency_id),
            currency_name: Set(user_wallet.currency_name),
            wallet_source_id: Set(user_wallet.wallet_source_id),
            wallet_source_name: Set(user_wallet.wallet_source_name),
            amount: Set(user_wallet.amount),
            create_at: NotSet,
            update_at: Set(chrono::Utc::now().naive_utc()),
        }
    }
}
