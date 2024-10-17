use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.create_table(manager).await?;
        self.create_index(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolloverMain::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RolloverMain {
    Table,
    Id,
    UserWalletId,
    ClientId,
    UserId,
    CurrencyId,
    CurrencyName,
    WalletSourceId,
    RequirementRollover,
    AchievementRollover,
    CreateAt,
    UpdateAt,
}

impl Migration {
    pub async fn create_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RolloverMain::Table)
                    .if_not_exists()
                    .col(big_integer(RolloverMain::Id).primary_key())
                    .col(big_integer(RolloverMain::UserWalletId).not_null())
                    .col(big_integer(RolloverMain::ClientId).not_null())
                    .col(big_integer(RolloverMain::UserId).not_null())
                    .col(big_integer(RolloverMain::CurrencyId).not_null())
                    .col(string(RolloverMain::CurrencyName).not_null())
                    .col(big_integer(RolloverMain::WalletSourceId).not_null())
                    .col(decimal(RolloverMain::RequirementRollover).not_null())
                    .col(decimal(RolloverMain::AchievementRollover).not_null())
                    .col(
                        timestamp(RolloverMain::CreateAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(RolloverMain::UpdateAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let comment = r#"
            COMMENT ON TABLE rollover_main IS '流水主表';
            COMMENT ON COLUMN rollover_main.id IS 'ID';
            COMMENT ON COLUMN rollover_main.user_wallet_id IS '對應的用戶錢包ID';
            COMMENT ON COLUMN rollover_main.client_id IS '用戶的client ID';
            COMMENT ON COLUMN rollover_main.user_id IS '用戶ID';
            COMMENT ON COLUMN rollover_main.currency_id IS '貨幣ID';
            COMMENT ON COLUMN rollover_main.currency_name IS '貨幣名稱';
            COMMENT ON COLUMN rollover_main.wallet_source_id IS '錢包來源ID';
            COMMENT ON COLUMN rollover_main.requirement_rollover IS '需求流水';
            COMMENT ON COLUMN rollover_main.achievement_rollover IS '達成流水';
            COMMENT ON COLUMN rollover_main.create_at IS '建立時間';
            COMMENT ON COLUMN rollover_main.update_at IS '更新時間';
        "#;

        manager.get_connection().execute_unprepared(comment).await?;
        Ok(())
    }

    pub async fn create_index(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .table(RolloverMain::Table)
                    .name("idx_rollover_main_user_wallet_id")
                    .col(RolloverMain::UserId)
                    .col(RolloverMain::ClientId)
                    .col(RolloverMain::CurrencyId)
                    .col(RolloverMain::WalletSourceId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
