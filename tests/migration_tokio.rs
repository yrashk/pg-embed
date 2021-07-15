#[cfg(feature = "sqlx_tokio")]
use sqlx_tokio::{Connection, PgConnection};
#[cfg(feature = "sqlx_async_std")]
use sqlx_async_std::{Connection, PgConnection};
#[cfg(feature = "sqlx_actix")]
use sqlx_actix::{Connection, PgConnection};
use serial_test::serial;
use pg_embed::pg_errors::PgEmbedError;
use std::path::PathBuf;
use std::env;

mod common;

#[tokio::test]
#[serial]
async fn db_creation() -> Result<(), PgEmbedError> {
    let mut pg = common::setup(5432, PathBuf::from("data_test/db"), false, None).await?;
    pg.start_db().await?;
    let db_name = "test";
    pg.create_database(&db_name).await?;
    assert!(pg.database_exists(&db_name).await?);
    Ok(())
}

#[tokio::test]
#[serial]
async fn db_migration() -> Result<(), PgEmbedError> {
    let mut pg = common::setup(
        5432,
        PathBuf::from("data_test/db"),
        false,
        Some(PathBuf::from("migration_test"))
    ).await?;
    pg.start_db().await?;
    let db_name = "test";
    pg.create_database(&db_name).await?;
    assert!(pg.database_exists(&db_name).await?);
    pg.migrate(&db_name).await?;

    let db_uri = pg.full_db_uri(&db_name);

    let mut conn = PgConnection::connect(&db_uri).await?;

    let _ = sqlx_tokio::query("INSERT INTO testing (description) VALUES ('Hello')")
        .execute(&mut conn)
        .await?;

    let row = sqlx_tokio::query("SELECT id, description, done FROM testing LIMIT 1")
        .fetch_one(&mut conn)
        .await?;

    Ok(())
}
