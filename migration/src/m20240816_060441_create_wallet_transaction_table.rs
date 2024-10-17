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
            .drop_table(Table::drop().table(WalletTransaction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WalletTransaction {
    Table,
    Id,
    ParentId,
    ClientId,
    UserId,
    CurrencyId,
    WalletSourceId,
    Action,
    TransactionSourceId,
    BeforeAmount,
    ChangeAmount,
    AfterAmount,
    Status,
    CreateAt,
    UpdateAt,
}

impl Migration {
    async fn create_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WalletTransaction::Table)
                    .if_not_exists()
                    .col(big_integer(WalletTransaction::Id).primary_key())
                    .col(big_integer(WalletTransaction::ParentId).not_null())
                    .col(big_integer(WalletTransaction::ClientId).not_null())
                    .col(big_integer(WalletTransaction::UserId).not_null())
                    .col(big_integer(WalletTransaction::CurrencyId).not_null())
                    .col(big_integer(WalletTransaction::WalletSourceId).not_null())
                    .col(integer(WalletTransaction::Action).not_null())
                    .col(big_integer(WalletTransaction::TransactionSourceId).not_null())
                    .col(decimal(WalletTransaction::BeforeAmount).not_null())
                    .col(decimal(WalletTransaction::ChangeAmount).not_null())
                    .col(decimal(WalletTransaction::AfterAmount).not_null())
                    .col(integer(WalletTransaction::Status).not_null())
                    .col(
                        timestamp(WalletTransaction::CreateAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(WalletTransaction::UpdateAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let comment = r#"
            COMMENT ON TABLE wallet_transaction IS '錢包交易紀錄';
            COMMENT ON COLUMN wallet_transaction.id IS 'ID';
            COMMENT ON COLUMN wallet_transaction.parent_id IS '關聯wallet_transaction的ID';
            COMMENT ON COLUMN wallet_transaction.client_id IS '用戶的client ID';
            COMMENT ON COLUMN wallet_transaction.user_id IS '用戶ID';
            COMMENT ON COLUMN wallet_transaction.currency_id IS '貨幣ID';
            COMMENT ON COLUMN wallet_transaction.wallet_source_id IS '錢包來源ID';
            COMMENT ON COLUMN wallet_transaction.action IS '交易行為 1:遊戲存款 2:遊戲提款 3:支付存款 4:支付提款';
            COMMENT ON COLUMN wallet_transaction.transaction_source_id IS '來源的txn ID';
            COMMENT ON COLUMN wallet_transaction.before_amount IS '交易前金額';
            COMMENT ON COLUMN wallet_transaction.change_amount IS '交易金額';
            COMMENT ON COLUMN wallet_transaction.after_amount IS '交易後金額';
            COMMENT ON COLUMN wallet_transaction.status IS '交易狀態 1:成功 2:失敗';
            COMMENT ON COLUMN wallet_transaction.create_at IS '建立時間';
            COMMENT ON COLUMN wallet_transaction.update_at IS '更新時間';
        "#;

        manager.get_connection().execute_unprepared(comment).await?;
        Ok(())
    }

    async fn create_index(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .table(WalletTransaction::Table)
                    .col(WalletTransaction::ClientId)
                    .col(WalletTransaction::UserId)
                    .col(WalletTransaction::CurrencyId)
                    .col(WalletTransaction::WalletSourceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
