
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use sqlx::PgPool;

use crate::{
    models::user::{CreateUserRequest, User},
    services::user_service,
};

pub async fn get_users(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<User>>, StatusCode> {

    let users = user_service::get_users(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(request): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {

    if request.name.trim().is_empty()
        || !request.email.contains('@')
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = user_service::create_user(
        &pool,
        request,
    )
        .await
        .map_err(|_| StatusCode::CONFLICT)?;

    Ok((StatusCode::CREATED, Json(user)))
}
