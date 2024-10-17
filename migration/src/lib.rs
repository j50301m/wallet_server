pub use sea_orm_migration::prelude::*;

mod m20240816_021302_crate_wallet_source_table;
mod m20240816_054942_create_user_wallet_table;
mod m20240816_060441_create_wallet_transaction_table;
mod m20240816_061738_create_rollover_main_table;
mod m20240816_062915_crate_rollover_record_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240816_021302_crate_wallet_source_table::Migration),
            Box::new(m20240816_054942_create_user_wallet_table::Migration),
            Box::new(m20240816_060441_create_wallet_transaction_table::Migration),
            Box::new(m20240816_061738_create_rollover_main_table::Migration),
            Box::new(m20240816_062915_crate_rollover_record_table::Migration),
        ]
    }
}
