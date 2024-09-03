#[derive(Clone)]
pub struct PostgresPool(sqlx::PgPool);

#[derive(Clone)]
pub struct TestPostgresPool(PostgresPool);

impl AsRef<sqlx::PgPool> for PostgresPool {
    #[inline]
    fn as_ref(&self) -> &sqlx::PgPool {
        &self.0
    }
}

impl AsRef<PostgresPool> for TestPostgresPool {
    #[inline]
    fn as_ref(&self) -> &PostgresPool {
        &self.0
    }
}

impl AsRef<sqlx::PgPool> for TestPostgresPool {
    #[inline]
    fn as_ref(&self) -> &sqlx::PgPool {
        self.0.as_ref()
    }
}

impl PostgresPool {
    pub async fn new(database_url: &'static str) -> Result<Self, super::Error> {
        sqlx::PgPool::connect(database_url)
            .await
            .map(Self)
            .map_err(super::Error::internal)
    }
}

impl TestPostgresPool {
    pub async fn new(database_url: &'static str) -> Result<Self, super::Error> {
        use std::sync::OnceLock;

        static POOL: OnceLock<TestPostgresPool> = OnceLock::new();

        match POOL.get() {
            Some(pool) => Ok(pool.clone()),
            None => {
                let pool = Self(PostgresPool::new(database_url).await?);
                POOL.set(pool.clone());

                Ok(pool)
            }
        }
    }
}

#[async_trait]
impl super::Connection for PostgresPool {
    type Connection = sqlx::pool::PoolConnection<sqlx::Postgres>;

    async fn connect(&self) -> Result<Self::Connection, super::Error> {
        self.0.acquire().await.map_err(super::Error::internal)
    }
}

#[async_trait]
impl super::Transaction for PostgresPool {
    type Transaction = sqlx::Transaction<'static, sqlx::Postgres>;

    async fn begin(&self) -> Result<Self::Transaction, super::Error> {
        self.0.begin().await.map_err(super::Error::internal)
    }

    async fn commit(connection: Self::Transaction) -> Result<(), super::Error> {
        connection.commit().await.map_err(super::Error::internal)
    }

    async fn rollback(connection: Self::Transaction) -> Result<(), super::Error> {
        connection.rollback().await.map_err(super::Error::internal)
    }
}
