use crate::error::Result;
use crate::types::{QueryResult, UndelegateResponse};
use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::BigDecimal;
use sqlx::Row;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct DelegatorUndelegateRecordsParams {
    pub delegator: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_delegator_undelegate_records(
    State(state): State<Arc<AppState>>,
    params: Query<DelegatorUndelegateRecordsParams>,
) -> Result<Json<QueryResult<Vec<UndelegateResponse>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let (sql_total, sql_query) = if let Some(delegator) = params.0.delegator {
        let delegator_lower = delegator.to_lowercase();
        (
            format!("SELECT count(block_num) FROM evm_undelegations WHERE delegator='{}'", delegator_lower)
            ,
            format!("SELECT tx_id,block_id,block_num,validator,delegator,tm,amount FROM evm_undelegations WHERE delegator='{}' ORDER BY tm DESC LIMIT {} OFFSET {}", delegator_lower, page_size, (page-1)*page_size)
        )
    } else {
        (
            "SELECT count(block_num) FROM evm_undelegations".to_string()
            ,
            format!("SELECT tx_id,block_id,block_num,validator,delegator,tm,amount FROM evm_undelegations ORDER BY tm DESC LIMIT {} OFFSET {}", page_size, (page-1)*page_size))
    };

    let row = sqlx::query(&sql_total).fetch_one(&mut *pool).await?;
    let total: i64 = row.try_get("count")?;

    let mut delegates: Vec<UndelegateResponse> = vec![];
    let rows = sqlx::query(&sql_query).fetch_all(&mut *pool).await?;
    for r in rows {
        let tx_hash: String = r.try_get("tx_id")?;
        let block_hash: String = r.try_get("block_id")?;
        let block_num: i64 = r.try_get("block_num")?;
        let validator: String = r.try_get("validator")?;
        let delegator: String = r.try_get("delegator")?;
        let amount: BigDecimal = r.try_get("amount")?;
        let tm: NaiveDateTime = r.try_get("tm")?;

        delegates.push(UndelegateResponse {
            tx_hash,
            block_hash,
            block_num,
            validator,
            delegator,
            amount: amount.to_string(),
            timestamp: tm.and_utc().timestamp(),
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: delegates,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct ValidatorUndelegateRecordsParams {
    pub validator: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn get_validator_undelegate_records(
    State(state): State<Arc<AppState>>,
    params: Query<ValidatorUndelegateRecordsParams>,
) -> Result<Json<QueryResult<Vec<UndelegateResponse>>>> {
    let mut pool = state.pool.acquire().await?;
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let (sql_total, sql_query) = if let Some(validator) = params.0.validator {
        (
            format!("SELECT count(block_num) FROM evm_undelegations WHERE validator='{}'", validator)
            ,
            format!("SELECT tx_id,block_id,block_num,validator,delegator,tm,amount FROM evm_undelegations WHERE validator='{}' ORDER BY tm DESC LIMIT {} OFFSET {}", validator, page_size, (page-1)*page_size)
        )
    } else {
        (
            "SELECT count(block_num) FROM evm_undelegations".to_string()
            ,
            format!("SELECT tx_id,block_id,block_num,validator,delegator,tm,amount FROM evm_undelegations ORDER BY tm DESC LIMIT {} OFFSET {}", page_size, (page-1)*page_size))
    };

    let row = sqlx::query(&sql_total).fetch_one(&mut *pool).await?;
    let total: i64 = row.try_get("count")?;

    let mut delegates: Vec<UndelegateResponse> = vec![];
    let rows = sqlx::query(&sql_query).fetch_all(&mut *pool).await?;
    for r in rows {
        let tx_hash: String = r.try_get("tx_id")?;
        let block_hash: String = r.try_get("block_id")?;
        let block_num: i64 = r.try_get("block_num")?;
        let validator: String = r.try_get("validator")?;
        let delegator: String = r.try_get("delegator")?;
        let amount: BigDecimal = r.try_get("amount")?;
        let tm: NaiveDateTime = r.try_get("tm")?;

        delegates.push(UndelegateResponse {
            tx_hash,
            block_hash,
            block_num,
            validator,
            delegator,
            amount: amount.to_string(),
            timestamp: tm.and_utc().timestamp(),
        })
    }

    Ok(Json(QueryResult {
        total,
        page,
        page_size,
        data: delegates,
    }))
}
