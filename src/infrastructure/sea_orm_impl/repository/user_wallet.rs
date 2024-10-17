use database_manager::Context;
use kgs_err::models::status::Status as KgsStatus;

use kgs_tracing::{tracing, warn};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::PgFunc;
use sea_orm::*;

use crate::domain::{self, *};
use crate::infrastructure::sea_orm_impl::aggregate::UserWalletWithRollover;
use crate::infrastructure::sea_orm_impl::entity::rollover_main;
use crate::infrastructure::sea_orm_impl::entity::user_wallet;

#[derive(Debug)]
pub struct UserWalletRepository;

#[tonic::async_trait]
impl UserWalletRepositoryTrait for UserWalletRepository {
    #[tracing::instrument]
    async fn get(&self, wallet_info: &WalletInfo) -> Result<Option<UserWallet>, KgsStatus> {
        let cx = Context::current();
        let txn = cx.get::<DatabaseTransaction>().ok_or_else(|| {
            warn!("get database transaction error");
            KgsStatus::InternalServerError
        })?;

        user_wallet::Entity::find()
            .filter(user_wallet::Column::ClientId.eq(wallet_info.client_id))
            .filter(user_wallet::Column::UserId.eq(wallet_info.user_id))
            .filter(user_wallet::Column::CurrencyId.eq(wallet_info.currency.id))
            .filter(user_wallet::Column::WalletSourceId.eq(wallet_info.wallet_source.id))
            .one(txn)
            .await
            .map(|entity| entity.map(|entity| entity.into()))
            .map_err(|e| {
                warn!("get user_wallet error: {:?}", e);
                KgsStatus::InternalServerError
            })
    }

    #[tracing::instrument]
    async fn get_user_wallets_with_rollover(
        &self,
        select_query: SelectWalletsQuery,
    ) -> Result<Vec<domain::UserWalletWithRollover>, KgsStatus> {
        let cx = Context::current();
        let txn = cx.get::<DatabaseTransaction>().ok_or_else(|| {
            warn!("get database transaction error");
            KgsStatus::InternalServerError
        })?;

        let mut query = user_wallet::Entity::find()
            .select_only()
            .column(user_wallet::Column::Id)
            .column(user_wallet::Column::ClientId)
            .column(user_wallet::Column::UserId)
            .column(user_wallet::Column::CurrencyId)
            .column(user_wallet::Column::CurrencyName)
            .column(user_wallet::Column::WalletSourceId)
            .column(user_wallet::Column::WalletSourceName)
            .column(user_wallet::Column::Amount)
            .column(rollover_main::Column::RequirementRollover)
            .column(rollover_main::Column::AchievementRollover)
            .filter(user_wallet::Column::ClientId.eq(select_query.client_id));

        if !select_query.player_ids.is_empty() {
            query = query.filter(
                Expr::col((user_wallet::Entity, user_wallet::Column::UserId))
                    .eq(PgFunc::any(select_query.player_ids)),
            );
        }
        if !select_query.currency_ids.is_empty() {
            query = query.filter(
                Expr::col((user_wallet::Entity, user_wallet::Column::CurrencyId))
                    .eq(PgFunc::any(select_query.currency_ids)),
            );
        }
        if !select_query.wallet_source_ids.is_empty() {
            query = query.filter(
                Expr::col((user_wallet::Entity, user_wallet::Column::WalletSourceId))
                    .eq(PgFunc::any(select_query.wallet_source_ids)),
            );
        }

        let result = query
            .join(
                sea_orm::JoinType::InnerJoin,
                user_wallet::Relation::RolloverMain.def(),
            )
            .order_by_desc(user_wallet::Column::UpdateAt)
            .into_model::<UserWalletWithRollover>()
            .paginate(txn, select_query.page_size)
            .fetch_page(select_query.page - 1)
            .await
            .map_err(|e| {
                warn!("get user_wallets_with_rollover error: {:?}", e);
                KgsStatus::InternalServerError
            })?;

        Ok(result
            .into_iter()
            .map(|entity: UserWalletWithRollover| entity.into())
            .collect())
    }

    #[tracing::instrument]
    async fn insert(&self, user_wallet: UserWallet) -> Result<UserWallet, KgsStatus> {
        let cx = Context::current();
        let txn = cx.get::<DatabaseTransaction>().ok_or_else(|| {
            warn!("get database transaction error");
            KgsStatus::InternalServerError
        })?;

        let active_model = user_wallet::ActiveModel::from(user_wallet);

        active_model
            .insert(txn)
            .await
            .map(|entity| entity.into())
            .map_err(|e| {
                warn!("insert user_wallet error: {:?}", e);
                KgsStatus::InternalServerError
            })
    }

    #[tracing::instrument]
    async fn update(&self, user_wallet: UserWallet) -> Result<UserWallet, KgsStatus> {
        let cx = Context::current();
        let txn = cx.get::<DatabaseTransaction>().ok_or_else(|| {
            warn!("get database transaction error");
            KgsStatus::InternalServerError
        })?;

        let active_model = user_wallet::ActiveModel::from(user_wallet);

        active_model
            .update(txn)
            .await
            .map(|entity| entity.into())
            .map_err(|e| {
                warn!("update user_wallet error: {:?}", e);
                KgsStatus::InternalServerError
            })
    }
}
