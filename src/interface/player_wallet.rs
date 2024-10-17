use kgs_tracing::tracing;
use protos::player_wallet;
use tonic::Response;

use crate::application;

#[derive(Debug)]
pub struct PlayerWalletService {
    player_wallet_app: application::UserWalletService,
}

impl PlayerWalletService {
    pub fn new(player_wallet_app: application::UserWalletService) -> PlayerWalletService {
        PlayerWalletService { player_wallet_app }
    }
}

#[tonic::async_trait]
impl player_wallet::player_wallet_server::PlayerWallet for PlayerWalletService {
    #[tracing::instrument]
    async fn get(
        &self,
        request: tonic::Request<player_wallet::PlayerWalletRequest>,
    ) -> Result<tonic::Response<player_wallet::WalletModel>, tonic::Status> {
        self.player_wallet_app
            .get(request.into_inner())
            .await
            .map(|res| Response::new(res))
            .map_err(|e| kgs_err::grpc::error::error_response(e))
    }

    #[tracing::instrument]
    async fn deposit(
        &self,
        request: tonic::Request<player_wallet::PlayerWalletChangeRequest>,
    ) -> Result<tonic::Response<player_wallet::WalletModel>, tonic::Status> {
        self.player_wallet_app
            .deposit(request.into_inner())
            .await
            .map(|res| Response::new(res))
            .map_err(|e| kgs_err::grpc::error::error_response(e))
    }

    #[tracing::instrument]
    async fn withdraw(
        &self,
        request: tonic::Request<player_wallet::PlayerWalletChangeRequest>,
    ) -> Result<tonic::Response<player_wallet::WalletModel>, tonic::Status> {
        self.player_wallet_app
            .withdraw(request.into_inner())
            .await
            .map(|res| Response::new(res))
            .map_err(|e| kgs_err::grpc::error::error_response(e))
    }

    #[tracing::instrument]
    async fn get_list(
        &self,
        request: tonic::Request<player_wallet::GetPlayerWalletListRequest>,
    ) -> Result<tonic::Response<player_wallet::GetPlayerWalletListResponse>, tonic::Status> {
        self.player_wallet_app
            .get_list(request.into_inner())
            .await
            .map(|res| Response::new(res))
            .map_err(|e| kgs_err::grpc::error::error_response(e))
    }

    #[tracing::instrument]
    async fn rollback(
        &self,
        request: tonic::Request<player_wallet::RollbackRequest>,
    ) -> Result<tonic::Response<player_wallet::WalletModel>, tonic::Status> {
        self.player_wallet_app
            .rollback(request.into_inner())
            .await
            .map(|res| Response::new(res))
            .map_err(|e| kgs_err::grpc::error::error_response(e))
    }

    #[tracing::instrument]
    async fn transfer(
        &self,
        request: tonic::Request<player_wallet::PlayerTransferRequest>,
    ) -> Result<tonic::Response<player_wallet::WalletModel>, tonic::Status> {
        // self.player_wallet_app
        //     .transfer(request.into_inner())
        //     .await
        //     .map(|res| Response::new(res))
        //     .map_err(|e| kgs_err::grpc::error::error_response(e))
        Err(tonic::Status::new(tonic::Code::NotFound, "尚未實作"))
    }
}
