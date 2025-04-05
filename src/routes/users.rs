use crate::db;
use crate::error::AppError;
use crate::models::{User, UserEmailRequest};
use axum::extract::{Json, Path, State};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_user_by_id(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    let user = db::get_user_by_id(&pool, user_id).await?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(AppError::UserNotFound),
    }
}
pub async fn get_current_user(
    State(pool): State<PgPool>,
    Json(payload): Json<UserEmailRequest>,
) -> Result<Json<User>, AppError> {
    let user = db::get_user_by_email(&pool, &payload.email).await?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(AppError::UserNotFound),
    }
}
