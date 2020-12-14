use crate::{db::GenClient, repository::UserRepository};

#[derive(Debug)]
pub enum RegistrationServiceError {
    UsernameAlreadyExists,
    TokioError(tokio_postgres::Error),
}

pub struct RegistrationService {}

impl RegistrationService {
    pub async fn register_user(
        &self,
        username: &str,
        user_repo: &dyn UserRepository,
        client: &mut (dyn GenClient + Send + Sync),
    ) -> Result<u64, RegistrationServiceError> {
        match user_repo.get_by_username(username, client).await {
            Some(_) => Err(RegistrationServiceError::UsernameAlreadyExists),
            None => user_repo
                .create_user(username, client)
                .await
                .map_err(RegistrationServiceError::TokioError),
        }
    }

    pub async fn register_users(
        &self,
        amount: u64,
        user_repo: &dyn UserRepository,
        client: &mut (dyn GenClient + Send + Sync),
    ) -> Result<u64, RegistrationServiceError> {
        let mut transaction = client.transaction().await.unwrap();

        for _ in 0..amount {
            user_repo
                .create_user(&uuid::Uuid::new_v4().to_string(), &mut transaction)
                .await
                .unwrap();
        }

        transaction.commit().await.unwrap();
        Ok(amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::MockClient, model::User, service::RegistrationService};
    use async_trait::async_trait;
    struct UserRepositoryMockImpl {}

    #[async_trait]
    impl UserRepository for UserRepositoryMockImpl {
        async fn get_by_username(
            &self,
            username: &str,
            _: &mut (dyn GenClient + Send + Sync),
        ) -> Option<User> {
            Some(User {
                id: 1,
                username: username.to_owned(),
            })
        }

        async fn create_user(
            &self,
            _: &str,
            _: &mut (dyn GenClient + Send + Sync),
        ) -> Result<u64, tokio_postgres::Error> {
            Ok(1)
        }
    }

    #[actix_rt::test]
    async fn test_registration_exists() {
        let user_repo = UserRepositoryMockImpl {};
        let reg_srv = RegistrationService {};

        let mut mock_clnt = MockClient {};

        let result = reg_srv
            .register_user("Hans", &user_repo, &mut mock_clnt)
            .await;
        let already_exists = match result {
            Err(e) => match e {
                RegistrationServiceError::UsernameAlreadyExists => true,
                _ => false,
            },
            Ok(_) => false,
        };

        assert_eq!(already_exists, true);
    }
}
