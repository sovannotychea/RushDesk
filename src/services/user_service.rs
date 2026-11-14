
use sqlx::PgPool;

use crate::{
    models::user::{CreateUserRequest, User},
    repositories::user_repository,
};

pub async fn get_users(
    pool: &PgPool,
) -> Result<Vec<User>, sqlx::Error> {
    user_repository::find_all(pool).await
}

pub async fn create_user(
    pool: &PgPool,
    request: CreateUserRequest,
) -> Result<User, sqlx::Error> {

    user_repository::save(
        pool,
        &request.name,
        &request.email,
    ).await
}
