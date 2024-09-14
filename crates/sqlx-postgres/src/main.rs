use sqlx::postgres::PgPoolOptions;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:root@localhost:5000/exchange-db")
        .await?;

    // insert_user(&pool).await?;

    let user = sqlx::query!("SELECT email, first_name, last_name, username FROM users")
        .fetch_one(&pool)
        .await?;

    println!("Got: {:?}", user);

    Ok(())
}

async fn insert_user(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO users(email, first_name, last_name, username) VALUES ($1, $2, $3, $4)",
    )
    .bind("test@test.com")
    .bind("test")
    .bind("last")
    .bind("user")
    .execute(pool)
    .await?;

    Ok(())
}
