use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use crate::errors::ServiceError;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use mobc_postgres::{tokio_postgres::{Row}};
use actix_multipart_extract::{File, MultipartForm};

#[derive(Debug, Deserialize)]
pub struct Query {
    pub limit: u32,
    pub offset: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PostgresMapper)]
#[pg_mapper(table = "reviewsdata")] 
pub struct Review {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub original: String,
    #[serde(rename="url")]
    pub thumbnail: String,
    pub web: String,
    pub deleted: bool,
   // pub created_at: chrono::NaiveDateTime,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewListInfo {
    pub reviews: Vec<Review>,
    pub reviews_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq/*, PostgresMapper*/)]
#[serde(rename_all = "camelCase")]
pub struct NewReview<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub original: &'a str,
    pub thumbnail: &'a str,
    pub web: &'a str,
    pub deleted: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputReview {
    pub title: String,
    pub description: String,
    pub original: String,
    pub thumbnail: String,
    pub web: String,
    pub deleted: bool,
}
#[derive(Deserialize, MultipartForm, Debug)]
pub struct ReviewForm {
    #[multipart(max_size = 12MB)]
    pub title: String,
    pub description: String, 
    pub picture: File,
//    pub deleted: bool, 
}
#[derive(Serialize, Deserialize)]
pub struct FileDownloaded {
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct Passphrase {
    pub passphrase: String,
}

impl Review {

    pub async fn insert(
        client: &Client,
        review: InputReview
    ) -> Result<Review, ServiceError> {
        let _stmt = include_str!("../../sql/review/add_review.sql");
        let _stmt = _stmt.replace("$table_fields", &Review::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();

        client
        .query(
            &stmt,
            &[
                &review.title,
                &review.description,
                &review.original,
                &review.thumbnail,
                &review.web,
                &review.deleted,
            ],
        )
        .await?
        .iter()
        .map(|row| Review::from_row_ref(row).unwrap())
        .collect::<Vec<Review>>()
        .pop()
        .ok_or(ServiceError::Unauthorized) // more applicable for SELECTs
    }
    pub async fn update(
        review_id: i32,
        review: InputReview,
        client: &Client ) -> Result<Review, ServiceError> {
        let _stmt = include_str!("../../sql/review/update_review.sql");
        let _stmt = _stmt.replace("$table_fields", &Review::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &review_id,
                    &review.title,
                    &review.description,
                ],
            )
            .await?
            .iter()
            .map(|row| Review::from_row_ref(row).unwrap())
            .collect::<Vec<Review>>()
            .pop()
            .ok_or(ServiceError::Unauthorized) // more applicable for SELECTs
    
    }
    pub async fn delete_review(review_id: i32, client: &Client) -> Result<Review, ServiceError> {
        let _stmt = include_str!("../../sql/review/delete_review.sql");
        let _stmt = _stmt.replace("$table_fields", &Review::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &review_id,
                ],
            )
            .await?
            .iter()
            .map(|row| Review::from_row_ref(row).unwrap())
            .collect::<Vec<Review>>()
            .pop()
            .ok_or(ServiceError::Unauthorized) // more applicable for SELECTs
    }

    pub async fn get_review_by_id(review_id: i32, client: &Client) -> Result<Review, ServiceError> {
        let _stmt = include_str!("../../sql/review/get_review_by_id.sql");
        let _stmt = _stmt.replace("$table_fields", &Review::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        client
            .query(
                &stmt,
                &[
                    &review_id,
                ],
            )
            .await?
            .iter()
            .map(|row| Review::from_row_ref(row).unwrap())
            .collect::<Vec<Review>>()
            .pop()
            .ok_or(ServiceError::Unauthorized) 
    }
    pub async fn get_reviews(client: &Client) -> Result<Vec<Review>, ServiceError> {
        let _stmt = include_str!("../../sql/review/get_reviews.sql");
        let _stmt = _stmt.replace("$table_fields", &Review::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        let rows = client.query(&stmt, &[]).await?;
        Ok(rows
            .into_iter()
            .map(|row| Review::from(row))
            .collect())
    }
}
impl From<Row> for Review {
    fn from(row: Row) -> Self {
        Self {
            id : row.get(0),
            title: row.get(1),
            description: row.get(2),
            original: row.get(3),
            thumbnail: row.get(4),
            web: row.get(5),
            deleted: row.get(6),
//            created_at: row.get(7),
        }
    }
}
