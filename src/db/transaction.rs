use chrono::{DateTime, Utc};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::TimestampSeconds;
use sqlx::{
    postgres::{PgListener, PgPoolOptions},
    FromRow, PgPool,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransactionType {
    Credit,
    Debit,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransactionType::Credit => write!(f, "c"),
            TransactionType::Debit => write!(f, "d"),
        }
    }
}

impl Serialize for TransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            TransactionType::Credit => serializer.serialize_str("c"),
            TransactionType::Debit => serializer.serialize_str("d"),
        }
    }
}

impl<'de> Deserialize<'de> for TransactionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        println!("OI");
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "c" => Ok(TransactionType::Credit),
            "d" => Ok(TransactionType::Debit),
            _ => Err(D::Error::custom("tipo => 'c' ou 'd'")),
        }
    }
}

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Transaction {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
    #[serde_as(as = "TimestampSeconds<String>")]
    pub realizada_em: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NewTransaction {
    pub id_cliente: i32,
    #[serde(deserialize_with = "deserialize_valor")]
    pub valor: i32,
    #[serde(deserialize_with = "deserialize_descricao")]
    pub descricao: String,
    pub tipo: TransactionType,
}

#[derive(Debug, Deserialize)]
pub struct NewTransaction2 {
    pub id_cliente: i32,
    pub valor: i32,
    pub descricao: String,
    pub tipo: String,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub saldo: i32,
    pub limite: i32,
}

fn deserialize_valor<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let num: i32 = i32::deserialize(deserializer)?;

    if num < 0 {
        Err(D::Error::custom("valor must be greater then zero"))
    } else {
        Ok(num)
    }
}

fn deserialize_descricao<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.len() < 1 || s.len() > 10 {
        Err(D::Error::custom("1 <= len(descricao) <= 10"))
    } else {
        Ok(s)
    }
}
