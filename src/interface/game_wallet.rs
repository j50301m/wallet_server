use std::fmt::Debug;

use crate::application;
use kgs_tracing::tracing;
use protos::game_wallet::*;

#[derive(Debug)]
pub struct GameWalletService {
    game_wallet_service: application::GameWalletService,
}

impl GameWalletService {
    pub fn new(game_wallet_service: application::GameWalletService) -> GameWalletService {
        GameWalletService {
            game_wallet_service,
        }
    }
}

#[tonic::async_trait]
impl game_wallet_server::GameWallet for GameWalletService {
    #[tracing::instrument]
    async fn get(
        &self,
        request: tonic::Request<BalanceRequest>,
    ) -> Result<tonic::Response<BalanceResponse>, tonic::Status> {
        self.game_wallet_service
            .get_balance(request.into_inner())
            .await
            .map(|res| tonic::Response::new(res))
            .map_err(|e| kgs_err::grpc::error::error_response(e))
    }

    #[tracing::instrument]
    async fn deposit(
        &self,
        request: tonic::Request<DepositRequest>,
    ) -> Result<tonic::Response<DepositResponse>, tonic::Status> {
        self.game_wallet_service
            .deposit(request.into_inner())
            .await
            .map(|res| tonic::Response::new(res))
            .map_err(|e| kgs_err::grpc::error::error_response(e))
    }

    #[tracing::instrument]
    async fn withdraw(
        &self,
        request: tonic::Request<WithdrawRequest>,
    ) -> Result<tonic::Response<WithdrawResponse>, tonic::Status> {
        self.game_wallet_service
            .withdraw(request.into_inner())
            .await
            .map(|res| tonic::Response::new(res))
            .map_err(|e| kgs_err::grpc::error::error_response(e))
    }

    #[tracing::instrument]
    async fn update(
        &self,
        request: tonic::Request<UpdateRequest>,
    ) -> Result<tonic::Response<UpdateResponse>, tonic::Status> {
        self.game_wallet_service
            .update(request.into_inner())
            .await
            .map(|res| tonic::Response::new(res))
            .map_err(|e| kgs_err::grpc::error::error_response(e))
    }

    #[tracing::instrument]
    async fn rollback(
        &self,
        request: tonic::Request<RollbackRequest>,
    ) -> Result<tonic::Response<RollbackResponse>, tonic::Status> {
        self.game_wallet_service
            .rollback(request.into_inner())
            .await
            .map(|res| tonic::Response::new(res))
            .map_err(|e| kgs_err::grpc::error::error_response(e))
    }
}
