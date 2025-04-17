use sea_orm::DatabaseConnection;
use crate::config::CONFIG;

pub mod entities;

pub async fn init_db() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let db = sea_orm::Database::connect(&CONFIG.database_url).await?;
    Ok(db)
}