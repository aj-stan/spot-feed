use crate::db::users::model::User;

use deadpool_postgres::Pool;
use uuid::Uuid;

pub async fn create_user(
    pool: &Pool,
    username: &str,
    email: &str,
    password: &str,
) -> Result<User, anyhow::Error> {
    let client = pool.get().await?;
    let user_id = Uuid::new_v4();

    // Skip hashing, store plain password (TODO: hash in future)
    let hash = password.to_string();

    let stmt = "
        INSERT INTO users (id, username, email, password_hash, is_guest)
        VALUES ($1, $2, $3, $4, false)
        RETURNING *
    ";
    let row = client
        .query_one(stmt, &[&user_id, &username, &email, &hash])
        .await?;
    Ok(User::from_row(&row))
}
