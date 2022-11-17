use super::models::{User,RegisterInfoWrapper,ProfileInfoWrapper,ListUserQuery,Counted,
    LoginInfoWrapper,UserInfoWrapper,UserUpdateInfoWrapper};
use actix_web::{get, post, delete, put , web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use crate::utils::{utils};
use actix_web_httpauth::extractors::bearer::BearerAuth;

#[post("/users")]
pub async fn add_user(
    user: web::Json<RegisterInfoWrapper>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let mut user_info: RegisterInfoWrapper = user.into_inner();
    let client: Client = db_pool.get().await.unwrap();
    user_info.user.password = utils::hash_password(&user_info.user.password.clone()).await.unwrap();
    let new_user = User::add_user(&client, user_info.user).await?;
    Ok(HttpResponse::Ok().json(UserInfoWrapper{ user: new_user,}))
}

#[post("/users/login")]
pub async fn login(
    login: web::Json<LoginInfoWrapper>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.unwrap();
    let mut user_info: LoginInfoWrapper = login.into_inner();
    let hashed_password = utils::hash_password(&user_info.user.password.clone()).await.unwrap();
    user_info.user.password = hashed_password;
    let logged = User::get_user_by_email_and_password(user_info.user,&client).await?;
    Ok(HttpResponse::Ok().json(UserInfoWrapper{ user: logged,}))
}

#[get("/users")]
pub async fn get_users(
    info: web::Query<ListUserQuery>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.unwrap();
    let users = User::get_users(&info,&client).await?;
    let counted = Counted {
        total: users.len() as u32,
        results: users,
    };
    Ok(HttpResponse::Ok().json(counted))
}
#[get("/user/{username}")]
pub async fn get_user(
    username: web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.unwrap();
    let user = User::get_user(username.into_inner(),&client).await?;
    Ok(HttpResponse::Ok().json(UserInfoWrapper{ user: user,}))
}

#[get("/user")]
pub async fn get_current_user(
    auth: BearerAuth,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.unwrap();
    let user = User::get_current_user(auth.token().to_string(),&client).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
pub async fn delete_user(  id: web::Path<i32>,db_pool: web::Data<Pool>) -> Result<HttpResponse,Error> {
        let client: Client = db_pool.get().await.unwrap();
        User::delete_user(id.into_inner(),&client).await?;
        Ok(HttpResponse::Ok().finish())
}
#[put("/users")]
pub async fn update_user(
            auth: BearerAuth,
            user: web::Json<UserUpdateInfoWrapper>,
            db_pool: web::Data<Pool>) -> Result<HttpResponse,Error> {
        let client: Client = db_pool.get().await.unwrap();
        let user_info: UserUpdateInfoWrapper = user.into_inner();
        let updated_user = User::update_user(auth.token().to_string(),user_info.user ,&client).await?;
        Ok(HttpResponse::Ok().json(UserInfoWrapper {user : updated_user,}))
}

#[get("/profiles/{username}")]
pub async fn get_profile(
    username: web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.unwrap();
    let profile = User::get_user_by_username(username.into_inner(),&client).await?;
    Ok(HttpResponse::Ok().json(ProfileInfoWrapper{ profile: profile,}))
}

#[get("/profiles/{user_id}")]
pub async fn get_user_profile(
    user_id: web::Path<i32>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.unwrap();
    let profile = User::get_user_profile(user_id.into_inner(),&client).await?;
    Ok(HttpResponse::Ok().json(ProfileInfoWrapper{ profile: profile,}))
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add_user);
    cfg.service(get_user);
    cfg.service(get_current_user);
    cfg.service(get_users);
    cfg.service(update_user);
    cfg.service(delete_user);
    cfg.service(login);
    cfg.service(get_profile);
    cfg.service(get_user_profile);
}
