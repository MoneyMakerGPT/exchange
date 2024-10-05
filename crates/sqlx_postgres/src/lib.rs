use sqlx::postgres::PgPoolOptions;

pub struct PostgresDb {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresDb {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://root:root@exchange-postgres:5432/exchange-db")
            .await?;

        Ok(Self { pool })
    }

    pub fn get_pg_connection(&self) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
        Ok(self.pool.clone())
    }
}
