use std::{cmp::max, time::Duration};

use anyhow::Ok;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
};

use crate::config;

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let db_config = config::get().database();

    let mut option = ConnectOptions::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.username(),
        db_config.password(),
        db_config.host(),
        db_config.port(),
        db_config.database(),
    ));
    let cpu_nums = num_cpus::get() as u32;
    option
        .min_connections(max(cpu_nums * 4, 10))
        .max_connections(max(cpu_nums * 8, 20))
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(60))
        .max_lifetime(Duration::from_secs(3600 * 24))
        .sqlx_logging(false)
        .set_schema_search_path(db_config.schema());

    let db = Database::connect(option).await?;
    db.ping().await?;
    tracing::info!("Database connected successfully!");

    log_database_version(&db).await?;
    Ok(db)
}

async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    let version_result = db
        .query_one(Statement::from_string(
            DbBackend::Postgres,
            String::from("SELECT version()"),
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to get database version"))?;

    tracing::info!(
        "Database version: {}",
        version_result.try_get_by_index::<String>(0)?
    );
    Ok(())
}
