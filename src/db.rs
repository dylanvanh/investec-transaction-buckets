use anyhow::Result;
use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePool},
};
use std::str::FromStr;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn initialize(database_url: &str) -> Result<Self> {
        let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        MIGRATOR.run(&pool).await?;

        Ok(Self { pool })
    }
}

pub async fn find_transaction_id_by_uuid(pool: &SqlitePool, uuid: &str) -> Result<Option<i64>> {
    let row: Option<(i64,)> =
        sqlx::query_as(r#"SELECT id FROM investec_transactions WHERE uuid = ?1 LIMIT 1"#)
            .bind(uuid)
            .fetch_optional(pool)
            .await?;

    Ok(row.map(|tuple| tuple.0))
}

pub async fn insert_tx_and_annotation(
    pool: &SqlitePool,
    tx: &crate::clients::investec::models::Transaction,
    bucket: &str,
    notes: Option<&str>,
) -> Result<i64> {
    let mut txn = pool.begin().await?;

    let insert_tx_result = sqlx::query(
        r#"
        INSERT INTO investec_transactions (
            account_id, tx_type, transaction_type, status, description,
            card_number, posted_order, posting_date, value_date, action_date,
            transaction_date, amount, running_balance, uuid
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
        "#,
    )
    .bind(&tx.account_id)
    .bind(&tx.type_)
    .bind(&tx.transaction_type)
    .bind(&tx.status)
    .bind(&tx.description)
    .bind(&tx.card_number)
    .bind(&tx.posted_order)
    .bind(&tx.posting_date)
    .bind(&tx.value_date)
    .bind(&tx.action_date)
    .bind(&tx.transaction_date)
    .bind(tx.amount)
    .bind(&tx.running_balance)
    .bind(&tx.uuid)
    .execute(&mut *txn)
    .await?;

    let inserted_id = insert_tx_result.last_insert_rowid();

    sqlx::query(
        r#"
        INSERT INTO transaction_annotations (
            investec_transaction_id, bucket, notes
        ) VALUES (?1, ?2, ?3)
        "#,
    )
    .bind(inserted_id)
    .bind(bucket)
    .bind(notes)
    .execute(&mut *txn)
    .await?;

    txn.commit().await?;

    Ok(inserted_id)
}
