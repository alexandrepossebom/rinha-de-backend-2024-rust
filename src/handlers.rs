use axum::{
    extract::{Path, State},
    Json,
};
use axum_macros::debug_handler;
use sqlx::{Pool, Postgres};
use time::OffsetDateTime;

use crate::{
    apperror::AppError,
    models::{Account, CreateTransaction, Summary, Transaction, TransactionReply},
};

#[debug_handler]
pub async fn add_transaction(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateTransaction>,
) -> Result<Json<TransactionReply>, AppError> {
    if (payload.kind != "c" && payload.kind != "d")
        || payload.amount < 1
        || payload.description.len() > 10
        || payload.description.is_empty()
    {
        return Err(AppError::UnprocessableEntity);
    }

    let amount = if payload.kind == "d" {
        -payload.amount
    } else {
        payload.amount
    };

    let rec = sqlx::query!(
        "UPDATE clientes SET saldo = saldo + $1 WHERE id = $2 RETURNING saldo, limite",
        amount,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        if e.as_database_error()
            .map_or(false, |e| e.constraint().is_some())
        {
            AppError::UnprocessableEntity
        } else {
            AppError::NotFound
        }
    })?;

    sqlx::query!(
        "INSERT INTO transacoes (valor, tipo, descricao, cliente_id) VALUES ( $1,$2,$3,$4 )",
        payload.amount,
        payload.kind,
        payload.description,
        id
    )
    .execute(&pool)
    .await
    .map_err(|_| AppError::NotFound)?;

    Ok(Json(TransactionReply {
        limite: rec.limite,
        saldo: rec.saldo,
    }))
}

#[debug_handler]
pub async fn get_account(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<Account>, AppError> {
    let rec = sqlx::query!("SELECT saldo, limite FROM clientes WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::NotFound)?;

    let summary = Summary {
        total: rec.saldo,
        data_extrato: OffsetDateTime::now_utc(),
        limite: rec.limite,
    };

    let transactions = sqlx::query_as!(Transaction,"SELECT valor, tipo, descricao, realizada_em FROM transacoes WHERE cliente_id = $1 ORDER BY id DESC LIMIT 10",id)
    .fetch_all(&pool)
    .await
    .map_err(|_| {AppError::NotFound})?;

    Ok(Json(Account {
        saldo: summary,
        ultimas_transacoes: transactions,
    }))
}
