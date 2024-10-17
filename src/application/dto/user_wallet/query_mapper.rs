use std::{fmt::Debug, sync::Arc};

use crate::domain::*;

use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::tracing;

#[tonic::async_trait]
pub trait QueryMapperTrait: Send + Sync + Debug {
    async fn to_select_wallet_query(
        &self,
        proto: protos::player_wallet::GetPlayerWalletListRequest,
    ) -> Result<SelectWalletsQuery, KgsStatus>;
}

#[derive(Debug)]
pub struct QueryMapper {
    currency_service: Arc<dyn CurrencyServiceTrait>,
}

impl QueryMapper {
    pub fn new(currency_service: Arc<dyn CurrencyServiceTrait>) -> Self {
        Self { currency_service }
    }
}

#[tonic::async_trait]
impl QueryMapperTrait for QueryMapper {
    #[tracing::instrument]
    async fn to_select_wallet_query(
        &self,
        proto: protos::player_wallet::GetPlayerWalletListRequest,
    ) -> Result<SelectWalletsQuery, KgsStatus> {
        let page = proto.page.unwrap_or_default().max(1) as u64;
        let page_size = proto.page_size.unwrap_or_default().max(25) as u64;

        let currency_list = self
            .currency_service
            .get_enable_currencies(proto.client_id, proto.currencies)
            .await?;
        let currency_ids = currency_list.into_iter().map(|c| c.id).collect();

        Ok(SelectWalletsQuery {
            client_id: proto.client_id,
            player_ids: proto.player_ids,
            currency_ids,
            wallet_source_ids: proto.wallet_sources,
            page,
            page_size,
        })
    }
}
