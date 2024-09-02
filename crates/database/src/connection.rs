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

pub enum Error {
    Internal(anyhow::Error),
}
