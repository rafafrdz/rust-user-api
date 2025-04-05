use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::time::Duration;
use uuid::Uuid;

pub async fn create_pool() -> anyhow::Result<PgPool> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await?;

    // Run migrations if needed
    //  sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

pub async fn get_user_by_id(
    pool: &PgPool,
    id: Uuid,
) -> anyhow::Result<Option<crate::models::User>> {
    let user = sqlx::query_as!(
        crate::models::User,
        r#"
        SELECT id, email
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_email(
    pool: &PgPool,
    email: &str,
) -> anyhow::Result<Option<crate::models::User>> {
    let user = sqlx::query_as!(
        crate::models::User,
        r#"
        SELECT id, email
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
