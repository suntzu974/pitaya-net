use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use deadpool_postgres::Client;
use crate::errors::ServiceError;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::Row;
use validator_derive::Validate;

#[derive(Deserialize, Serialize,  Debug, Clone)]
pub struct Counted<T> {
    pub total: u32,
    pub results: Vec<T>,
}
#[derive(Debug, Deserialize, Validate)]
#[serde(default)]
pub struct CommentQuery {
    #[validate(range(min = 1, max = 100))]
    limit: i64,
    offset: i64,
    search: Option<String>,
}

impl Default for CommentQuery {
    fn default() -> Self {
        CommentQuery {
            limit: 20,
            offset: 0,
            search: None,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PostgresMapper)]
#[serde(rename_all = "camelCase")]
#[pg_mapper(table = "comments")] 
pub struct Comment {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub body: String,
    pub slug: String,
    pub author: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CommentWrapper {
    pub comment: Comment,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default,PostgresMapper)]
#[serde(rename_all = "camelCase")]
#[pg_mapper(table = "comments")] 
pub struct CommentCreate {
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommentCreateWrapper {
    pub comment: CommentCreate,
}


impl Comment {

    pub async fn add_comment(client:&Client,comment_info:CommentCreate,
        article_info:String,profile_info:String) -> Result <Comment,ServiceError> {
        let _stmt = include_str!("../../sql/comment/add_comment.sql");
        let _stmt = _stmt.replace("$table_fields", &Comment::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        let row = client
            .query_one(
                &stmt,
                    &[
                        &comment_info.body,
                        &article_info,
                        &profile_info,
                    ],
            ).await?;
            Ok(Comment::from(row))
    }

    pub async fn get_comments(info:&CommentQuery,slug:String,client: &Client) 
        -> Result<Vec<Comment>, ServiceError> {
        let _stmt = include_str!("../../sql/comment/get_comments.sql");
        let stmt = client.prepare(&_stmt).await.unwrap();
        let rows = client.query(&stmt, &[&(info.limit as i64),&(info.offset as i64),&slug]).await?;
        Ok(rows
            .into_iter()
            .map(|row| Comment::from(row))
            .collect())
    }
}

impl From<Row> for Comment {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0) ,
            created_at: row.get(1),
            updated_at: row.get(2),
            body: row.get(3),
            slug: row.get(4),
            author: row.get(5),
        }
    }
}
