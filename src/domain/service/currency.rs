use std::fmt::Debug;
use std::vec;

use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::tracing;

use crate::domain;
use crate::enums;
use crate::infrastructure;

#[tonic::async_trait]
pub trait CurrencyServiceTrait: Send + Sync + Debug {
    async fn get_enable_currencies(
        &self,
        client_id: i64,
        currency_names: Vec<String>,
    ) -> Result<Vec<domain::Currency>, KgsStatus>;

    async fn get_enable_currency(
        &self,
        client_id: i64,
        currency_name: &str,
    ) -> Result<domain::Currency, KgsStatus>;

    async fn get_enable_currency_by_id(
        &self,
        client_id: i64,
        currency_id: i64,
    ) -> Result<domain::Currency, KgsStatus>;
}

#[derive(Debug)]
pub struct CurrencyService;

/// 實作CurrencyServiceTrait
#[tonic::async_trait]
impl CurrencyServiceTrait for CurrencyService {
    #[tracing::instrument]
    async fn get_enable_currencies(
        &self,
        client_id: i64,
        currency_names: Vec<String>,
    ) -> Result<Vec<domain::Currency>, KgsStatus> {
        let res = infrastructure::bank_server::get_client_currencies(
            0,
            client_id,
            vec![enums::CurrencyStatus::Enable],
            currency_names,
        )
        .await?;

        let currencies = res
            .currency
            .into_iter()
            .map(|x| domain::Currency {
                id: x.id,
                name: x.name,
            })
            .collect();

        Ok(currencies)
    }

    #[tracing::instrument]
    async fn get_enable_currency(
        &self,
        client_id: i64,
        currency_name: &str,
    ) -> Result<domain::Currency, KgsStatus> {
        self.get_enable_currencies(client_id, vec![currency_name.to_string()])
            .await?
            .into_iter()
            .next()
            .ok_or(KgsStatus::NotFound)
    }

    #[tracing::instrument]
    async fn get_enable_currency_by_id(
        &self,
        client_id: i64,
        currency_id: i64,
    ) -> Result<domain::Currency, KgsStatus> {
        let res =
            infrastructure::bank_server::get_client_currency_by_id(client_id, currency_id).await?;

        if res.status != enums::CurrencyStatus::Enable.to_id() {
            return Err(KgsStatus::NotFound);
        }

        Ok(domain::Currency {
            id: res.id,
            name: res.name,
        })
    }
}
