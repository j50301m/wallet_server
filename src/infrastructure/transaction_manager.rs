// use std::sync::Arc;

// // use sqlx_core::transaction;
// use tokio::sync::Mutex;
// use kgs_tracing::warn;
// use sea_orm::{DatabaseConnection, DatabaseTransaction};
// use kgs_err::models::status::Status as KgsStatus;

// #[tonic::async_trait]
// pub trait TransactionManagerTrait {
//     async fn get_transaction(&self) -> &DatabaseTransaction;
//     async fn commit(mut self) -> Result<(),KgsStatus>;
//     async fn rollback(mut self) -> Result<(), KgsStatus>;
// }

// pub struct TransactionManager {
//     transaction
// }

// impl TransactionManager {
//     pub async fn build() -> Result<Self,KgsStatus> {
//         let transaction = database_manager::sea_orm::get_trans().await
//         .map_err(|e|{
//             warn!("get_transaction error: {:?}", e);
//             KgsStatus::InternalServerError
//         })?;

//         Ok(Self{
//             transaction
//         })
//     }
// }

// #[tonic::async_trait]
// impl TransactionManagerTrait for TransactionManager {
//     async fn get_transaction(&self) -> &DatabaseTransaction{
//         &self.transaction

//     }

//     async fn commit(mut self) -> Result<(), KgsStatus> {
//         self.transaction.commit().await
//         .map_err(|e|{
//             warn!("commit error: {:?}", e);
//             KgsStatus::InternalServerError
//         })
//     }

//     async fn rollback(mut self) -> Result<(), KgsStatus> {
//         self.transaction.rollback().await
//         .map_err(|e|{
//             warn!("rollback error: {:?}", e);
//             KgsStatus::InternalServerError
//         })
//     }
// }
