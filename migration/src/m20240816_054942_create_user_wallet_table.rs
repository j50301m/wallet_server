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
            .drop_table(Table::drop().table(UserWallet::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserWallet {
    Table,
    Id,
    ClientId,
    UserId,
    CurrencyId,
    CurrencyName,
    WalletSourceId,
    WalletSourceName,
    Amount,
    CreateAt,
    UpdateAt,
}

impl Migration {
    async fn create_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserWallet::Table)
                    .if_not_exists()
                    .col(big_integer(UserWallet::Id).primary_key())
                    .col(big_integer(UserWallet::ClientId).not_null())
                    .col(big_integer(UserWallet::UserId).not_null())
                    .col(big_integer(UserWallet::CurrencyId).not_null())
                    .col(string(UserWallet::CurrencyName).not_null())
                    .col(big_integer(UserWallet::WalletSourceId).not_null())
                    .col(string(UserWallet::WalletSourceName).not_null())
                    .col(decimal(UserWallet::Amount).not_null())
                    .col(
                        timestamp(UserWallet::CreateAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(UserWallet::UpdateAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        let comment = r#"
            COMMENT ON TABLE user_wallet IS '用戶錢包';
            COMMENT ON COLUMN user_wallet.id IS 'ID';
            COMMENT ON COLUMN user_wallet.client_id IS '用戶的client ID';
            COMMENT ON COLUMN user_wallet.user_id IS '用戶ID';
            COMMENT ON COLUMN user_wallet.currency_id IS '貨幣ID';
            COMMENT ON COLUMN user_wallet.currency_name IS '貨幣名稱';
            COMMENT ON COLUMN user_wallet.wallet_source_id IS '錢包來源ID';
            COMMENT ON COLUMN user_wallet.wallet_source_name IS '錢包來源名稱';
            COMMENT ON COLUMN user_wallet.amount IS '金額';
            COMMENT ON COLUMN user_wallet.create_at IS '建立時間';
            COMMENT ON COLUMN user_wallet.update_at IS '更新時間';
        "#;

        manager.get_connection().execute_unprepared(comment).await?;
        Ok(())
    }

    async fn create_index(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .table(UserWallet::Table)
                    .col(UserWallet::ClientId)
                    .col(UserWallet::UserId)
                    .col(UserWallet::WalletSourceId)
                    .col(UserWallet::CurrencyId)
                    .unique()
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
