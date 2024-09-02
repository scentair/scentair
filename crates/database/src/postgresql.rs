pub struct PostgreSqlPool {
    pool: sqlx::PgPool,
}

impl PostgreSqlPool {
    pub async fn new(database_url: &str) -> Result<Self, super::Error> {
        match sqlx::PgPool::connect(database_url).await {
            Ok(pool) => Ok(Self { pool }),
            Err(error) => Err(super::Error::Internal(error.into())),
        }
    }
}

#[async_trait]
impl super::Connection for PostgreSqlPool {
    type Connection = sqlx::pool::PoolConnection<sqlx::Postgres>;

    async fn connect(&self) -> Result<Self::Connection, super::Error> {
        match self.pool.acquire().await {
            Ok(connection) => Ok(connection),
            Err(error) => Err(super::Error::Internal(error.into())),
        }
    }
}

#[async_trait]
impl super::Transaction for PostgreSqlPool {
    type Transaction = sqlx::Transaction<'static, sqlx::Postgres>;

    async fn begin(&self) -> Result<Self::Transaction, super::Error> {
        match self.pool.begin().await {
            Ok(connection) => Ok(connection),
            Err(error) => Err(super::Error::Internal(error.into())),
        }
    }

    async fn commit(connection: Self::Transaction) -> Result<(), super::Error> {
        match connection.commit().await {
            Ok(()) => Ok(()),
            Err(error) => Err(super::Error::Internal(error.into())),
        }
    }

    async fn rollback(connection: Self::Transaction) -> Result<(), super::Error> {
        match connection.rollback().await {
            Ok(()) => Ok(()),
            Err(error) => Err(super::Error::Internal(error.into())),
        }
    }
}
