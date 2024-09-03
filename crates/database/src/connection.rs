#[async_trait]
pub trait Connection {
    type Connection;

    async fn connect(&self) -> Result<Self::Connection, Error>;
}

#[async_trait]
pub trait Transaction {
    type Transaction;

    async fn begin(&self) -> Result<Self::Transaction, Error>;

    async fn commit(connection: Self::Transaction) -> Result<(), Error>;

    async fn rollback(connection: Self::Transaction) -> Result<(), Error>;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Internal error")]
    Internal(anyhow::Error),
}

impl Error {
    #[inline]
    pub fn internal<E: Into<anyhow::Error>>(error: E) -> Self {
        Self::Internal(error.into())
    }
}
