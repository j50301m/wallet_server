use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::{tracing, warn};
use protos::client_currency::client_currency_client::ClientCurrencyClient;
use protos::client_currency::CurrencyModel;
use protos::client_currency::GetByIdRequest;
use protos::client_currency::GetByNameRequest;

use crate::config;

#[tracing::instrument]
async fn get_client() -> Result<ClientCurrencyClient<tonic::transport::Channel>, KgsStatus> {
    let config = config::get_bank_server();
    let addr = format!("{}:{}", config.bank_server_host, config.bank_server_port);
    let client = ClientCurrencyClient::connect(addr).await.map_err(|e| {
        warn!("connect bank server err:{:#?}", e);
        KgsStatus::InternalServerError
    })?;

    Ok(client)
}

#[tracing::instrument]
pub async fn get_client_currency_by_id(
    client_id: i64,
    currency_id: i64,
) -> Result<CurrencyModel, KgsStatus> {
    let body = GetByIdRequest {
        client_id,
        currency_id,
    };

    let mut client = get_client().await?;

    let request = kgs_tracing::tonic::create_request_with_span(body);

    let res = client.get_by_id(request).await.map_err(|e| {
        warn!("get client currency by id err:{:#?}", e);
        kgs_err::models::status::msg_2_status(e.message())
    })?;

    Ok(res.into_inner())
}

#[tracing::instrument]
pub async fn get_client_currency_by_name(
    client_id: i64,
    currency_name: &str,
) -> Result<CurrencyModel, KgsStatus> {
    let body = GetByNameRequest {
        client_id,
        name: currency_name.to_string(),
    };

    let mut client = get_client().await?;

    let request = kgs_tracing::tonic::create_request_with_span(body);

    let res = client.get_by_name(request).await.map_err(|e| {
        warn!("get client currency by name err:{:#?}", e);
        kgs_err::models::status::msg_2_status(e.message())
    })?;

    Ok(res.into_inner())
}
