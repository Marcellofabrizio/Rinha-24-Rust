use crate::db::transaction;
use axum::{http::StatusCode, Json};
use sqlx::{
    postgres::{PgListener, PgPoolOptions},
    PgPool, Row,
};
use std::{error::Error, sync::Arc};

use super::transaction::{NewTransaction, Transaction};

pub enum PersistenceError {
    UniqueViolation,
    DatabaseError(Box<dyn Error + Send + Sync>),
}

impl From<sqlx::Error> for PersistenceError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(err) if err.is_unique_violation() => {
                PersistenceError::UniqueViolation
            }
            _ => PersistenceError::DatabaseError(Box::new(error)),
        }
    }
}

type PersistenceResult<T> = Result<T, PersistenceError>;

pub struct PgRepository {
    pool: PgPool,
}

impl PgRepository {
    pub async fn connect(url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new().connect(url).await?;

        tokio::spawn({
            let pool = pool.clone();

            async move {
                if let Ok(mut listener) = PgListener::connect_with(&pool).await {
                    listener.listen("transaction_created").await.ok();
                    while let Ok(msg) = listener.recv().await {
                        if let Ok(transaction) = serde_json::from_str::<Transaction>(msg.payload())
                        {
                            println!("Transaction Created");
                        }
                    }
                }
            }
        });

        Ok(PgRepository { pool })
    }

    pub async fn create_transaction(
        &self,
        id_cliente: i32,
        new_transaction: NewTransaction,
    ) -> Result<StatusCode, sqlx::Error> {
        let mut db_transaction = self.pool.begin().await?;

        let client = sqlx::query!("SELECT * FROM clientes WHERE id = $1", id_cliente)
            .fetch_one(&mut *db_transaction)
            .await
            .map(|row| row.id);

        println!("{:?}", client);

        match client {
            Ok(_) => {
                println!("Adding new saldo");

                let new_saldo = sqlx::query!(
                    "SELECT novo_saldo FROM add_transaction($1, $2, $3, $4)",
                    id_cliente,
                    new_transaction.valor,
                    new_transaction.descricao,
                    new_transaction.tipo.to_string()
                )
                .fetch_one(&mut *db_transaction)
                .await;

                println!("Novo Saldo: {:?}", new_saldo);
                return Ok(StatusCode::ACCEPTED);
            }
            Err(err) => {
                &db_transaction.commit();
                return Err(err);
            }
        }
    }
}
