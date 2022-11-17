use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use crate::errors::ServiceError;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use mobc_postgres::{tokio_postgres::{Row}};
use validator_derive::Validate;
use uuid::{Uuid};

#[derive(Deserialize, Serialize,  Debug, Clone)]
pub struct Counted<T> {
    pub total: u32,
    pub results: Vec<T>,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(default)]
pub struct ListUserQuery {
    #[validate(range(min = 1, max = 100))]
    limit: i64,
    offset: i64,
    search: Option<String>,
}

impl Default for ListUserQuery {
    fn default() -> Self {
        ListUserQuery {
            limit: 20,
            offset: 0,
            search: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug,PostgresMapper)]
#[pg_mapper(table = "users")] 
pub struct User {
    pub slug: String,
    pub username: Option<String>,
    pub email: String,
    pub password: String,
    pub token: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginInfoWrapper {
    pub user: LoginInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default,PostgresMapper)]
#[serde(rename_all = "camelCase")]
#[pg_mapper(table = "users")] 
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInfoWrapper {
    pub user: RegisterInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default,PostgresMapper)]
#[serde(rename_all = "camelCase")]
#[pg_mapper(table = "users")] 
pub struct UserInfo {
    pub username: String,
    pub email: String,
    pub token: String,
    pub bio: Option<String>,
    pub image: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoWrapper {
    pub user: UserInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default,PostgresMapper)]
#[serde(rename_all = "camelCase")]
#[pg_mapper(table = "users")] 
pub struct UserUpdateInfo {
    pub email: String,
    pub username: String,
    pub image: String,
    pub bio: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateInfoWrapper {
    pub user: UserUpdateInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq,PostgresMapper)]
#[serde(rename_all = "camelCase")]
#[pg_mapper(table = "users")] 
pub struct ProfileInfo {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileInfoWrapper {
    pub profile: ProfileInfo,
}

impl User {

    pub async fn add_user(client: &Client, user_info: RegisterInfo) -> Result<UserInfo, ServiceError> {
        let _stmt = include_str!("../../sql/user/add_user.sql");
        let _stmt = _stmt.replace("$table_fields", &UserInfo::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();

        client
            .query(
                &stmt,
                &[
                    &Uuid::new_v4().to_string(),
                    &user_info.username,
                    &user_info.email,
                    &user_info.password,
                ],
            )
            .await?
            .iter()
            .map(|row| UserInfo::from_row_ref(row).unwrap())
            .collect::<Vec<UserInfo>>()
            .pop()
            .ok_or(ServiceError::Unauthorized) // more applicable for SELECTs
    }
    pub async fn update_user(auth:String,user_info: UserUpdateInfo, client: &Client) -> Result<UserInfo, ServiceError> {

        let _stmt = include_str!("../../sql/user/update_user.sql");
        let _stmt = _stmt.replace("$table_fields", &UserInfo::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        client.query(&stmt, 
            &[&user_info.email,
            &user_info.username,
            &user_info.image,
            &user_info.bio,
            &auth,
            ])
        .await?
        .iter()
        .map(|row| UserInfo::from_row_ref(row).unwrap())
        .collect::<Vec<UserInfo>>()
        .pop()
        .ok_or(ServiceError::Unauthorized) // more applicable for SELECTs

    }
    pub async fn get_users(info: &ListUserQuery,client: &Client) -> Result<Vec<UserInfo>, ServiceError> {
        let mut query = String::new();
        match info.search.clone() {
            Some(s) => {
                query = format!("%{}%",s);
            }
            None => {}
        }
        let _stmt = include_str!("../../sql/user/get_users.sql");
        let _stmt = _stmt.replace("$table_fields", &UserInfo::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        let rows = client.query(&stmt, &[&query]).await?;
        Ok(rows
            .into_iter()
            .map(|row| UserInfo::from(row))
            .collect())
    }
    pub async fn get_user_by_email_and_password(session: LoginInfo, client: &Client) -> Result<UserInfo,ServiceError> {
        let _stmt = include_str!("../../sql/user/get_user_by_email_and_password.sql");
        let _stmt = _stmt.replace("$table_fields", &UserInfo::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &session.email,
                    &session.password,
                ],
            )
            .await?
            .iter()
            .map(|row| UserInfo::from_row_ref(row).unwrap())
            .collect::<Vec<UserInfo>>()
            .pop()
            .ok_or(ServiceError::WrongEmailOrPassword) // more applicable for SELECTs
    }

/*    pub async fn get_user_by_email(email: String, client: &Client) -> Result<UserInfo,ServiceError> {
        let _stmt = include_str!("../../sql/user/get_user_by_email.sql");
        let _stmt = _stmt.replace("$table_fields", &UserInfo::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &email,
                ],
            )
            .await?
            .iter()
            .map(|row| UserInfo::from_row_ref(row).unwrap())
            .collect::<Vec<UserInfo>>()
            .pop()
            .ok_or(ServiceError::Unauthorized) // more applicable for SELECTs
    }
*/
    pub async fn get_user(username: String, client: &Client) -> Result<UserInfo,ServiceError> {
        let _stmt = include_str!("../../sql/user/get_user_by_username.sql");
        let _stmt = _stmt.replace("$table_fields", &UserInfo::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &username,
                ],
            )
            .await?
            .iter()
            .map(|row| UserInfo::from_row_ref(row).unwrap())
            .collect::<Vec<UserInfo>>()
            .pop()
            .ok_or(ServiceError::Unauthorized) // more applicable for SELECTs
    }
    pub async fn get_current_user(token: String, client: &Client) -> Result<UserInfo,ServiceError> {
        let _stmt = include_str!("../../sql/user/get_current_user_by_token.sql");
        let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &token,
                ],
            )
            .await?
            .iter()
            .map(|row| UserInfo::from_row_ref(row).unwrap())
            .collect::<Vec<UserInfo>>()
            .pop()
            .ok_or(ServiceError::Unauthorized) // more applicable for SELECTs

    }




    pub async fn delete_user(user_id:i32,client: &Client) -> Result<(), ServiceError> {
        let _stmt = include_str!("../../sql/user/delete_user.sql");
        let stmt = client.prepare(&_stmt).await.unwrap();
        let _res =client.query(&stmt, &[&user_id]).await?;
        Ok(())
    }

    pub async fn get_user_by_username(username: String, client: &Client) -> Result<ProfileInfo,ServiceError> {
        let _stmt = include_str!("../../sql/user/get_user_by_username.sql");
        let _stmt = _stmt.replace("$table_fields", &ProfileInfo::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &username,
                ],
            )
            .await?
            .iter()
            .map(|row| ProfileInfo::from_row_ref(row).unwrap())
            .collect::<Vec<ProfileInfo>>()
            .pop()
            .ok_or(ServiceError::Unauthorized) // more applicable for SELECTs
    }

    pub async fn get_user_profile(user_id: i32, client: &Client) -> Result<ProfileInfo,ServiceError> {
        let _stmt = include_str!("../../sql/user/get_user_profile.sql");
        let _stmt = _stmt.replace("$table_fields", &ProfileInfo::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &user_id,
                ],
            )
            .await?
            .iter()
            .map(|row| ProfileInfo::from_row_ref(row).unwrap())
            .collect::<Vec<ProfileInfo>>()
            .pop()
            .ok_or(ServiceError::Unauthorized) // more applicable for SELECTs
    }


}

impl From<Row> for UserInfo {
    fn from(row: Row) -> Self {
        Self {
            username: row.get(0),
            email: row.get(1),
            token: row.get(2),
            bio: row.get(3),
            image: row.get(4),
        }
    }
}


impl From<Row> for ProfileInfo {
    fn from(row: Row) -> Self {
        Self {
            username: row.get(0),
            bio: row.get(1),
            image: row.get(2),
            following: row.get(3),
        }
    }
}
