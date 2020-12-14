use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "app_user")]
pub struct User {
    pub id: i64,
    pub username: String,
}
