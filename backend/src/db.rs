use sqlx::{Pool, Postgres};

pub type PgPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}
