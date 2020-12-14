use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::Pool;

use crate::{
    repository::{UserRepository, UserRepositoryImpl},
    service::{RegistrationService, RegistrationServiceError},
};

pub async fn get_user_handler(
    path: web::Path<(String,)>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let mut client = db_pool.get().await.unwrap();
    let user_repo = UserRepositoryImpl {};
    let user = user_repo.get_by_username(&path.0 .0, &mut **client).await;
    HttpResponse::Ok().json(user)
}

pub async fn create_user_handler(
    path: web::Path<(String,)>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let mut client = db_pool.get().await.unwrap();
    let user_repo = UserRepositoryImpl {};
    let reg_srv = RegistrationService {};

    match reg_srv
        .register_user(&path.0 .0, &user_repo, &mut **client)
        .await
    {
        Ok(count) => HttpResponse::Created().json(format!("{}", count)),
        Err(e) => match e {
            RegistrationServiceError::UsernameAlreadyExists => {
                HttpResponse::Conflict().json("Already exists")
            }
            _ => HttpResponse::InternalServerError().json(""),
        },
    }
}

pub async fn create_users_handler(
    path: web::Path<(u64,)>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let mut client = db_pool.get().await.unwrap();
    let user_repo = UserRepositoryImpl {};
    let reg_srv = RegistrationService {};

    match reg_srv
        .register_users(path.0 .0, &user_repo, &mut **client)
        .await
    {
        Ok(count) => HttpResponse::Created().json(count),
        Err(e) => HttpResponse::InternalServerError().json(format!("{:#?}", e)),
    }
}
