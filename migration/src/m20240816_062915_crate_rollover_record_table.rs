use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        self.craete_table(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolloverRecord::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RolloverRecord {
    Table,
    Id,
    MainId,
    ClientId,
    UserId,
    RequirementRollover,
    RequirementRolloverRate,
    AchievementRollover,
    AchievementRolloverRate,
    CreateBy,
    WalletTxnId,
    CreateAt,
}

impl Migration {
    async fn craete_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RolloverRecord::Table)
                    .if_not_exists()
                    .col(big_integer(RolloverRecord::Id).primary_key())
                    .col(big_integer(RolloverRecord::MainId).not_null())
                    .col(big_integer(RolloverRecord::ClientId).not_null())
                    .col(big_integer(RolloverRecord::UserId).not_null())
                    .col(decimal(RolloverRecord::RequirementRollover).not_null())
                    .col(decimal(RolloverRecord::RequirementRolloverRate).not_null())
                    .col(decimal(RolloverRecord::AchievementRollover).not_null())
                    .col(decimal(RolloverRecord::AchievementRolloverRate).not_null())
                    .col(big_integer(RolloverRecord::CreateBy).not_null())
                    .col(big_integer(RolloverRecord::WalletTxnId).not_null())
                    .col(
                        timestamp(RolloverRecord::CreateAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let comment = r#"
            COMMENT ON TABLE rollover_record IS '流水紀錄表';
            COMMENT ON COLUMN rollover_record.id IS 'ID';
            COMMENT ON COLUMN rollover_record.main_id IS '對應的流水主表ID';
            COMMENT ON COLUMN rollover_record.client_id IS '用戶的client ID';
            COMMENT ON COLUMN rollover_record.user_id IS '用戶ID';
            COMMENT ON COLUMN rollover_record.requirement_rollover IS '更動的需求流水(有正負)';
            COMMENT ON COLUMN rollover_record.requirement_rollover_rate IS '需求流水比例';
            COMMENT ON COLUMN rollover_record.achievement_rollover IS '更動的達成流水(有正負)';
            COMMENT ON COLUMN rollover_record.achievement_rollover_rate IS '達成流水比例';
            COMMENT ON COLUMN rollover_record.create_by IS '建立者';
            COMMENT ON COLUMN rollover_record.wallet_txn_id IS '錢包交易ID';
            COMMENT ON COLUMN rollover_record.create_at IS '建立時間';
        "#;

        manager.get_connection().execute_unprepared(comment).await?;
        Ok(())
    }
}
