use async_trait::async_trait;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{db::GenClient, model::User};

#[async_trait]
pub trait UserRepository {
    async fn create_user(
        &self,
        username: &str,
        client: &mut (dyn GenClient + Send + Sync),
    ) -> Result<u64, tokio_postgres::Error>;

    async fn get_by_username(
        &self,
        username: &str,
        client: &mut (dyn GenClient + Send + Sync),
    ) -> Option<User>;
}

pub struct UserRepositoryImpl {}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(
        &self,
        username: &str,
        client: &mut (dyn GenClient + Send + Sync),
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = client
            .prepare("INSERT INTO public.app_user(id, username) VALUES (DEFAULT, $1);")
            .await?;
        client.execute(&stmt, &[&username]).await
    }

    async fn get_by_username(
        &self,
        username: &str,
        client: &mut (dyn GenClient + Send + Sync),
    ) -> Option<User> {
        let stmt = client
            .prepare("SELECT * FROM app_user WHERE username = $1")
            .await
            .unwrap();
        if let Some(row) = client.query_opt(&stmt, &[&username]).await.unwrap() {
            Some(User::from_row(row).unwrap())
        } else {
            None
        }
    }
}
