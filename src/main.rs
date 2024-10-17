use std::{sync::Arc, time::Duration};

use context::common::context::Context;
use kgs_tracing::{info, tracing};
use tokio;
use tonic::transport::Server;

mod application;
mod config;
mod domain;
mod enums;
mod infrastructure;
mod interface;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_telemetry();
    // init_wallet_db().await;

    wallet_grpc_server().await?;

    tokio::signal::ctrl_c().await.unwrap();
    Ok(())
}

#[tracing::instrument]
fn init_telemetry() {
    let host_config = config::get_host();
    let telemetry_config = config::get_telemetry();

    kgs_tracing::TelemetryBuilder::new(&host_config.service_name)
        .enable_log(&telemetry_config.loki_url)
        .enable_metrics(&telemetry_config.otlp_url)
        .enable_tracing(&telemetry_config.otlp_url)
        .build();

    // start metrics system CPU and RAM
    kgs_tracing::components::base_metrics::base_metrics(&host_config.service_name);
    info!("telemetry init success");
}

#[tracing::instrument]
async fn init_wallet_db() {
    let config = config::get_wallet_db();
    let _db = database_manager::sea_orm::Builder::new()
        .db_user(&config.wallet_db_user)
        .db_password(&config.wallet_db_password)
        .db_host(&config.wallet_db_host)
        .db_port(&config.wallet_db_port)
        .db_name(&config.wallet_db_name)
        .max_connections(config.wallet_db_max_connection)
        .min_connections(config.wallet_db_min_connection)
        .logging(true)
        .logging_level(database_manager::sea_orm::LogLevel::Info)
        .build()
        .await;
}

#[tracing::instrument]
fn declare_service() -> (
    Arc<interface::PlayerWalletService>,
    Arc<interface::GameWalletService>,
) {
    use crate::domain::*;
    use crate::infrastructure::sea_orm_impl::repository::*;

    // repository
    let user_wallet_repo: Arc<dyn UserWalletRepositoryTrait> = Arc::new(UserWalletRepository);
    let wallet_source_repo: Arc<dyn WalletSourceRepositoryTrait> = Arc::new(WalletSourceRepository);
    let wallet_txn_repo: Arc<dyn WalletTransactionRepositoryTrait> =
        Arc::new(WalletTransactionRepository);
    let main_rollover_repo: Arc<dyn RolloverMainRepositoryTrait> = Arc::new(RolloverMainRepository);
    let rollover_record_repo: Arc<dyn RolloverRecordRepositoryTrait> =
        Arc::new(RolloverRecordRepository);

    // domain service
    let currency_service: Arc<dyn CurrencyServiceTrait> = Arc::new(CurrencyService);
    let wallet_service: Arc<dyn WalletServiceTrait> = Arc::new(WalletService::new(
        wallet_txn_repo.clone(),
        user_wallet_repo.clone(),
    ));
    let rollover_service: Arc<dyn RolloverServiceTrait> = Arc::new(RolloverService::new(
        main_rollover_repo.clone(),
        rollover_record_repo.clone(),
    ));

    // application mapper
    let wallet_mapper: Arc<dyn application::WalletMapperTrait> = Arc::new(
        application::WalletMapper::new(currency_service.clone(), wallet_source_repo.clone()),
    );
    let query_mapper: Arc<dyn application::QueryMapperTrait> =
        Arc::new(application::QueryMapper::new(currency_service.clone()));

    // application service
    let player_wallet_service = application::UserWalletService::new(
        user_wallet_repo.clone(),
        wallet_source_repo.clone(),
        wallet_service.clone(),
        rollover_service.clone(),
        wallet_mapper.clone(),
        query_mapper.clone(),
        currency_service.clone(),
    );
    let game_wallet_service = application::GameWalletService::new(
        wallet_source_repo.clone(),
        wallet_service.clone(),
        rollover_service.clone(),
        wallet_mapper.clone(),
    );

    // api
    let user_wallet_api = Arc::new(interface::PlayerWalletService::new(player_wallet_service));
    let game_wallet_api = Arc::new(interface::GameWalletService::new(game_wallet_service));

    (user_wallet_api, game_wallet_api)
}

#[tracing::instrument]
async fn wallet_grpc_server() -> Result<(), tonic::transport::Error> {
    use protos::game_wallet::game_wallet_server::GameWalletServer;
    use protos::player_wallet::player_wallet_server::PlayerWalletServer;

    let host_config = config::get_host();
    let addr = format!("{}:{}", host_config.service_host, host_config.service_port)
        .parse()
        .unwrap();

    let (player_wallet_api, game_wallet_api) = declare_service();

    info!("wallet grpc server start on {:?}", addr);

    let db = context::common::db_impl::SeaPostgresBuilder::default()
        .db_host("localhost")
        .db_port("5432")
        .db_user("admin")
        .db_password("admin")
        .db_name("wallet")
        .max_connections(100)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(60))
        .sqlx_logging(true)
        .sqlx_logging_level(context::common::db_impl::LogLevel::Info)
        .build()
        .await;

    let context = Context::current().with_value(db);

    Server::builder()
        .layer(kgs_tracing::middlewares::tonic::root_span_builder())
        .layer(kgs_tracing::middlewares::tonic::TracingRecord::default())
        .layer(context::common::context_middleware::ContextHolder::new(
            context,
        ))
        .add_service(PlayerWalletServer::from_arc(player_wallet_api))
        .add_service(GameWalletServer::from_arc(game_wallet_api))
        .serve(addr)
        .await
}
