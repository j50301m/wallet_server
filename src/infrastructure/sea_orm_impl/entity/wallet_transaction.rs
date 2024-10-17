use sea_orm::{entity::prelude::*, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "wallet_transaction")]

pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub parent_id: i64, // 關聯單的id
    pub client_id: i64,
    pub user_id: i64,
    pub currency_id: i64,
    pub wallet_source_id: i64,
    pub action: i32,
    pub transaction_source_id: i64,
    pub before_amount: BigDecimal,
    pub change_amount: BigDecimal,
    pub after_amount: BigDecimal,
    pub status: i32,
    pub create_at: chrono::NaiveDateTime,
    pub update_at: chrono::NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    WalletSource,
    RolloverRecord,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::WalletSource => Entity::belongs_to(super::wallet_source::Entity)
                .from(Column::WalletSourceId)
                .to(super::wallet_source::Column::Id)
                .into(),
            Self::RolloverRecord => Entity::has_one(super::rollover_record::Entity)
                .from(Column::Id)
                .to(super::rollover_record::Column::WalletTxnId)
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

impl ActiveModelBehavior for ActiveModel {}

impl Into<crate::domain::WalletTransaction> for Model {
    fn into(self) -> crate::domain::WalletTransaction {
        crate::domain::WalletTransaction {
            id: self.id,
            parent_id: self.parent_id,
            client_id: self.client_id,
            user_id: self.user_id,
            currency_id: self.currency_id,
            wallet_source_id: self.wallet_source_id,
            action: self.action,
            transaction_source_id: self.transaction_source_id,
            before_amount: self.before_amount,
            change_amount: self.change_amount,
            after_amount: self.after_amount,
            status: self.status,
            create_at: self.create_at,
            update_at: self.update_at,
        }
    }
}

impl From<crate::domain::WalletTransaction> for ActiveModel {
    fn from(domain: crate::domain::WalletTransaction) -> Self {
        Self {
            id: Set(domain.id),
            parent_id: Set(domain.parent_id),
            client_id: Set(domain.client_id),
            user_id: Set(domain.user_id),
            currency_id: Set(domain.currency_id),
            wallet_source_id: Set(domain.wallet_source_id),
            action: Set(domain.action),
            transaction_source_id: Set(domain.transaction_source_id),
            before_amount: Set(domain.before_amount),
            change_amount: Set(domain.change_amount),
            after_amount: Set(domain.after_amount),
            status: Set(domain.status),
            create_at: Set(domain.create_at),
            update_at: Set(domain.update_at),
        }
    }
}
