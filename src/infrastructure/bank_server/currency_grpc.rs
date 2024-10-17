use kgs_err::models::status::Status as KgsStatus;
use kgs_tracing::{tracing, warn};

use crate::config;
use crate::enums;
use protos::currency::{currency_client::CurrencyClient, GetRequest, GetResponse};

#[tracing::instrument]
async fn get_client() -> Result<CurrencyClient<tonic::transport::Channel>, KgsStatus> {
    let config = config::get_bank_server();
    let addr = format!("{}:{}", config.bank_server_host, config.bank_server_port);
    let client = CurrencyClient::connect(addr).await.map_err(|e| {
        warn!("connect bank server err:{:#?}", e);
        KgsStatus::InternalServerError
    })?;

    Ok(client)
}

#[tracing::instrument]
pub async fn get_client_currencies(
    business_id: i64,
    client_id: i64,
    status: Vec<enums::CurrencyStatus>,
    currency_names: Vec<String>,
) -> Result<GetResponse, KgsStatus> {
    let status = status.into_iter().map(|x| x.to_id()).collect();

    let body = GetRequest {
        client: client_id,
        business: business_id,
        currency: currency_names,
        status,
    };

    let mut client = get_client().await?;

    let request = kgs_tracing::tonic::create_request_with_span(body);

    let res = client.get(request).await.map_err(|e| {
        warn!("get enable client enable currencies err:{:#?}", e);
        kgs_err::models::status::msg_2_status(e.message())
    })?;

    Ok(res.into_inner())
}
