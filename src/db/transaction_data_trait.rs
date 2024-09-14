// use crate::models::transaction;
// use crate::{db::Database};
// use actix_web::web::Data;
// use async_trait::async_trait;
// use surrealdb::{Error};
//
// #[async_trait]
// pub trait TransactionDataTrait {
//     async fn get_all_transactions(db: &Data<Database>) -> Option<Vec<transaction>>;
//     async fn add_transaction(db: &Data<Database>, new_transaction: transaction) -> Option<transaction>;
//     async fn update_transaction(db: &Data<Database>, hash: String, updated_tx: transaction) -> Option<transaction>;
// }
//
// #[async_trait]
// impl TransactionDataTrait for Database {
//     async fn get_all_transactions(db: &Data<Database>) -> Option<Vec<transaction>> {
//         let result = db
//             .client
//             .select("transactions")
//             .await;
//
//         match result {
//             Ok(all_transactions) => Some(all_transactions),
//             Err(_) => None,
//         }
//     }
//
//     async fn add_transaction(db: &Data<Database>, new_transaction: transaction) -> Option<transaction> {
//         let created_transaction = db
//             .client
//             .create(("transactions", new_transaction.hash.clone()))
//             .content(new_transaction)
//             .await;
//
//         match created_transaction {
//             Ok(created) => created,
//             Err(_) => None,
//         }
//     }
//
//     async fn update_transaction(db: &Data<Database>, hash: String, updated_tx: transaction) -> Option<transaction> {
//         let find_transaction: Result<Option<transaction>, Error> = db
//             .client
//             .select(("transactions", &hash))
//             .await;
//
//         match find_transaction {
//             Ok(found) => {
//                 match found {
//                     Some(_found_transaction) => {
//                         let updated_transaction = db
//                             .client
//                             .update(("transactions", &hash))
//                             .content(updated_tx)
//                             .await;
//                         match updated_transaction {
//                             Ok(updated) => updated,
//                             Err(_) => None,
//                         }
//                     }
//                     None => None,
//                 }
//             }
//             Err(_) => None,
//         }
//     }
// }
