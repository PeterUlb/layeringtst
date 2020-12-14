use async_trait::async_trait;
//use deadpool_postgres::Transaction;
use tokio_postgres::{
    types::{ToSql, Type},
    GenericClient,
};
use tokio_postgres::{Row, Statement, ToStatement, Transaction};

/// Object Safe Wrapper Trait over GenericClient
/// Missing: _raw methods (TODO: handle associated types)
#[async_trait]
pub trait GenClient {
    async fn execute(
        &self,
        statement: &(dyn ToStatement + Sync + Send),
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, tokio_postgres::Error>;

    async fn query(
        &self,
        statement: &(dyn ToStatement + Sync + Send),
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, tokio_postgres::Error>;

    async fn query_one(
        &self,
        statement: &(dyn ToStatement + Sync + Send),
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, tokio_postgres::Error>;

    async fn query_opt(
        &self,
        statement: &(dyn ToStatement + Sync + Send),
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, tokio_postgres::Error>;

    async fn prepare(&self, query: &str) -> Result<Statement, tokio_postgres::Error>;

    async fn prepare_typed(
        &self,
        query: &str,
        parameter_types: &[Type],
    ) -> Result<Statement, tokio_postgres::Error>;

    async fn transaction(&mut self) -> Result<Transaction<'_>, tokio_postgres::Error>;
}

#[async_trait]
impl<T> GenClient for T
where
    T: GenericClient + Send + Sync,
{
    async fn execute(
        &self,
        statement: &(dyn ToStatement + Sync + Send),
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, tokio_postgres::Error> {
        self.execute(statement, params).await
    }

    async fn query(
        &self,
        statement: &(dyn ToStatement + Sync + Send),
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, tokio_postgres::Error> {
        self.query(statement, params).await
    }

    async fn query_one(
        &self,
        statement: &(dyn ToStatement + Sync + Send),
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, tokio_postgres::Error> {
        self.query_one(statement, params).await
    }

    async fn query_opt(
        &self,
        statement: &(dyn ToStatement + Sync + Send),
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, tokio_postgres::Error> {
        self.query_opt(statement, params).await
    }

    async fn prepare(&self, query: &str) -> Result<Statement, tokio_postgres::Error> {
        self.prepare(query).await
    }

    async fn prepare_typed(
        &self,
        query: &str,
        parameter_types: &[Type],
    ) -> Result<Statement, tokio_postgres::Error> {
        self.prepare_typed(query, parameter_types).await
    }

    async fn transaction(&mut self) -> Result<Transaction<'_>, tokio_postgres::Error> {
        self.transaction().await
    }
}

pub struct MockClient {}
#[async_trait]
impl GenClient for MockClient {
    async fn execute(
        &self,
        _: &(dyn ToStatement + Sync + Send),
        _: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, tokio_postgres::Error> {
        unimplemented!()
    }

    async fn query(
        &self,
        _: &(dyn ToStatement + Sync + Send),
        _: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, tokio_postgres::Error> {
        unimplemented!()
    }

    async fn query_one(
        &self,
        _: &(dyn ToStatement + Sync + Send),
        _: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, tokio_postgres::Error> {
        unimplemented!()
    }

    async fn query_opt(
        &self,
        _: &(dyn ToStatement + Sync + Send),
        _: &[&(dyn ToSql + Sync)],
    ) -> Result<Option<Row>, tokio_postgres::Error> {
        unimplemented!()
    }

    async fn prepare(&self, _: &str) -> Result<Statement, tokio_postgres::Error> {
        unimplemented!()
    }

    async fn prepare_typed(&self, _: &str, _: &[Type]) -> Result<Statement, tokio_postgres::Error> {
        unimplemented!()
    }

    #[allow(clippy::needless_lifetimes)]
    async fn transaction<'a>(&'a mut self) -> Result<Transaction<'a>, tokio_postgres::Error> {
        unimplemented!()
    }
}
