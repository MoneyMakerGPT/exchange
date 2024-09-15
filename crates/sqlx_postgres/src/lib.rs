use sqlx::postgres::PgPoolOptions;

pub struct PostgresDb {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresDb {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://root:root@localhost:5000/exchange-db")
            .await?;

        Ok(Self { pool })
    }

    pub fn get_pg_connection(&self) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
        Ok(self.pool.clone())
    }

    pub async fn insert_user(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO users(email, first_name, last_name, username) VALUES ($1, $2, $3, $4)",
        )
        .bind("test@test.com")
        .bind("test")
        .bind("last")
        .bind("user")
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user(&self) -> Result<String, sqlx::Error> {
        let user = sqlx::query!("SELECT * FROM users")
            .fetch_one(&self.pool)
            .await?;

        Ok(user.last_name)
    }
}
