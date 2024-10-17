use dotenv::dotenv;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

static CONFIG: Lazy<Arc<Config>> = Lazy::new(|| Config::init());

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub telemetry: Telemetry,
    pub host: Host,
    pub wallet_db: WalletDb,
    pub bank_server: BankServer,
    pub rabbitmq: RabbitMQ,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Telemetry {
    pub loki_url: String,
    pub otlp_url: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Host {
    pub service_name: String,
    pub service_host: String,
    pub service_port: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RabbitMQ {
    pub rabbitmq_host: String,
    pub rabbitmq_port: usize,
    pub rabbitmq_user: String,
    pub rabbitmq_password: String,
    pub rabbitmq_max_connection: usize,
    pub rabbitmq_min_connection: usize,
    pub rabbitmq_connection_timeout: usize,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WalletDb {
    pub wallet_db_host: String,
    pub wallet_db_port: String,
    pub wallet_db_user: String,
    pub wallet_db_password: String,
    pub wallet_db_name: String,
    pub wallet_db_max_connection: u32,
    pub wallet_db_min_connection: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BankServer {
    pub bank_server_host: String,
    pub bank_server_port: String,
}

impl Config {
    fn init() -> Arc<Config> {
        dotenv().ok();

        let host = envy::from_env::<Host>().expect("載入Host 環境變數失敗");
        let telemetry = envy::from_env::<Telemetry>().expect("載入Telemetry 環境變數失敗");
        let wallet_db = envy::from_env::<WalletDb>().expect("載入WalletDb 環境變數失敗");
        let bank_server = envy::from_env::<BankServer>().expect("載入BankServer 環境變數失敗");
        let rabbitmq = envy::from_env::<RabbitMQ>().expect("載入RabbitMQ 環境變數失敗");

        let config = Config {
            telemetry,
            host,
            wallet_db,
            bank_server,
            rabbitmq,
        };

        Arc::new(config)
    }
}

pub fn get_telemetry() -> &'static Telemetry {
    &CONFIG.telemetry
}

pub fn get_wallet_db() -> &'static WalletDb {
    &CONFIG.wallet_db
}

pub fn get_host() -> &'static Host {
    &CONFIG.host
}

pub fn get_bank_server() -> &'static BankServer {
    &CONFIG.bank_server
}

pub fn get_rabbit() -> &'static RabbitMQ {
    &CONFIG.rabbitmq
}
