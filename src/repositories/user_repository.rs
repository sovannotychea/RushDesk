
use sqlx::PgPool;

use crate::models::user::User;

pub async fn find_all(pool: &PgPool)
                      -> Result<Vec<User>, sqlx::Error>
{
    sqlx::query_as::<_, User>(
        "SELECT id, name, email FROM users ORDER BY id"
    )
        .fetch_all(pool)
        .await
}

pub async fn save(
    pool: &PgPool,
    name: &str,
    email: &str,
) -> Result<User, sqlx::Error> {

    sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email)
         VALUES ($1, $2)
         RETURNING id, name, email"
    )
        .bind(name)
        .bind(email)
        .fetch_one(pool)
        .await
}
