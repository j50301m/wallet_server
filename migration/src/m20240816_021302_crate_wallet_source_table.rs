use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.create_table(manager).await?;
        self.create_data(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WalletSource::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WalletSource {
    Table,
    Id,
    Name,
    CreateAt,
}

impl Migration {
    async fn create_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WalletSource::Table)
                    .if_not_exists()
                    .col(big_integer(WalletSource::Id).primary_key())
                    .col(string(WalletSource::Name).not_null())
                    .col(
                        timestamp(WalletSource::CreateAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let comment = r#"
            COMMENT ON TABLE wallet_source IS '錢包來源';
            COMMENT ON COLUMN wallet_source.id IS 'ID';
            COMMENT ON COLUMN wallet_source.name IS '名稱';
            COMMENT ON COLUMN wallet_source.create_at IS '建立時間';
        "#;

        manager.get_connection().execute_unprepared(comment).await?;
        Ok(())
    }

    async fn create_data(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        let sql = r#"
            INSERT INTO wallet_source (id, name, create_at) VALUES
            (1, 'normal_wallet', now()),
            (2, 'bonus_wallet', now());
        "#;
        manager.get_connection().execute_unprepared(sql).await?;
        Ok(())
    }
}
