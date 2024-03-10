use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Deserialize)]
pub struct CreateTransaction {
    #[serde(rename = "valor")]
    pub amount: i32,
    #[serde(rename = "tipo")]
    pub kind: String,
    #[serde(rename = "descricao")]
    pub description: String,
}

#[derive(Serialize)]
pub struct TransactionReply {
    pub limite: i32,
    pub saldo: i32,
}

#[derive(Serialize)]
pub struct Transaction {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
    #[serde(with = "time::serde::rfc3339")]
    pub realizada_em: OffsetDateTime,
}

#[derive(Serialize)]
pub struct Account {
    pub saldo: Summary,
    pub ultimas_transacoes: Vec<Transaction>,
}

#[derive(Serialize)]
pub struct Summary {
    pub total: i32,
    #[serde(with = "time::serde::rfc3339")]
    pub data_extrato: OffsetDateTime,
    pub limite: i32,
}
