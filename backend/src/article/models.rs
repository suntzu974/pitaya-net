use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use deadpool_postgres::Client;
use crate::errors::ServiceError;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;
use crate::user::models::{ProfileInfo};
use uuid::{Uuid};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq/*, PostgresMapper*/)]
#[serde(rename_all = "camelCase")]
//#[pg_mapper(table = "articles")] 
pub struct ArticleInfoWithProfile {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    pub favorites_count: i32,
    pub author: ProfileInfo,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, PostgresMapper)]
#[serde(rename_all = "camelCase")]
#[pg_mapper(table = "articles")] 
pub struct ArticleInfo {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub favorited: bool,
    pub favorites_count: i32,
    pub author: String,

}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArticleInfoWithProfileWrapper {
    pub article: ArticleInfoWithProfile,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArticleInfoWrapper {
    pub article: ArticleInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticleListInfo {
    pub articles: Vec<ArticleInfoWithProfile>,
    pub articles_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default,PostgresMapper)]
#[serde(rename_all = "camelCase")]
#[pg_mapper(table = "articles")] 
pub struct ArticleCreateUpdateInfo {
    pub title: String,
    pub description: String,
    pub body: String,
//    #[serde(rename(deserialize="tags"))]
    pub tag_list: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticleCreateUpdateInfoWrapper {
    pub article: ArticleCreateUpdateInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagListInfo {
    pub tags: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CommentInfo {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub body: String,
    pub author: ProfileInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CommentInfoWrapper {
    pub comment: CommentInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CommentCreateInfo {
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommentCreateInfoWrapper {
    pub comment: CommentCreateInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommentListInfo {
    pub comments: Vec<CommentInfo>,
}

#[derive(Debug, Deserialize)]
pub struct Query {
    pub limit: u32,
    pub offset: u32,
//    search: Option<String>,
}
impl ArticleInfoWithProfile {

    pub async fn add_article(client:&Client,article_info:ArticleCreateUpdateInfo,
        profile_info:String) -> Result < ArticleInfoWithProfile,ServiceError> {
        let _stmt = include_str!("../../sql/article/add_article.sql");
//        let _stmt = _stmt.replace("$table_fields", &ArticleInfo::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();
        let row = client
            .query_one(
                &stmt,
                    &[
                        &Uuid::new_v4().to_string(),
                        &article_info.title,
                        &article_info.description,
                        &article_info.body,
                        &article_info.tag_list,
                        &profile_info,
                    ],
            ).await?;
            Ok(ArticleInfoWithProfile::from(row))
    }

    pub async fn update_article(slug:String,client:&Client,
        article_info:ArticleCreateUpdateInfo) -> Result < ArticleInfoWithProfile,ServiceError> {
        let _stmt = include_str!("../../sql/article/update_article.sql");
//        let _stmt = _stmt.replace("$table_fields", &ArticleInfo::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();

        let row = client
            .query_one(
                &stmt,
                    &[
                        &article_info.title,
                        &article_info.description,
                        &article_info.body,
                        &article_info.tag_list,
                        &slug,
                    ],
            ).await?;
            
        Ok(ArticleInfoWithProfile::from(row))
    }

    pub async fn get_article(slug:String,client: &Client) -> Result<ArticleInfoWithProfile, ServiceError> {
        let _stmt = include_str!("../../sql/article/get_article.sql");
        let stmt = client.prepare(&_stmt).await.unwrap();
        let row = client.query_one(&stmt, &[&slug,]).await?;
        
        Ok(ArticleInfoWithProfile::from(row))
    }

    pub async fn get_feed_articles(auth:String,info:&Query,client: &Client) -> Result<Vec<ArticleInfoWithProfile>, ServiceError> {
        let _stmt = include_str!("../../sql/article/get_feed_articles.sql");
        let stmt = client.prepare(&_stmt).await.unwrap();
        let rows = client.query(&stmt, &[&(info.limit as i64),&(info.offset as i64),&auth]).await?;
        Ok(rows
            .into_iter()
            .map(|row| ArticleInfoWithProfile::from(row))
            .collect())
    }

    pub async fn get_articles(info:&Query,client: &Client) -> Result<Vec<ArticleInfoWithProfile>, ServiceError> {
        let _stmt = include_str!("../../sql/article/get_articles.sql");
        let stmt = client.prepare(&_stmt).await.unwrap();
        let rows = client.query(&stmt, &[&(info.limit as i64),&(info.offset as i64),]).await?;
        Ok(rows
            .into_iter()
            .map(|row| ArticleInfoWithProfile::from(row))
            .collect())
    }
    pub async fn get_tags(client: &Client) -> Result<Vec<TagListInfo>, ServiceError> {
        let _stmt = include_str!("../../sql/article/get_tags.sql");
        let stmt = client.prepare(&_stmt).await.unwrap();
        let rows = client.query(&stmt, &[]).await?;
        Ok(rows
            .into_iter()
            .map(|row| TagListInfo::from(row))
            .collect())
 
    }
    pub async fn get_comments(slug:String,client: &Client) -> Result<Vec<CommentInfo>, ServiceError> {
        let _stmt = include_str!("../../sql/article/get_comments.sql");
        let stmt = client.prepare(&_stmt).await.unwrap();
        let rows = client.query(&stmt, &[&slug]).await?;
        Ok(rows
            .into_iter()
            .map(|row| CommentInfo::from(row))
            .collect())
 
    }

    pub async fn add_comment(client:&Client,comment_info:CommentCreateInfo,
        article_info:String,profile_info:String) -> Result <CommentInfo,ServiceError> {
        let _stmt = include_str!("../../sql/article/add_comment.sql");
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
            Ok(CommentInfo::from(row))
    }


}

impl From<Row> for ArticleInfoWithProfile {
    fn from(row: Row) -> Self {
        Self {
            slug: row.get(0) ,
            title: row.get(1),
            description: row.get(2),
            body: row.get(3),
            tag_list: row.get(4),
            created_at: row.get(5),
            updated_at: row.get(6),
            favorited: row.get(7),
            favorites_count: row.get(8),
            author: ProfileInfo {
                username:row.get(9),
                bio: row.get(10),
                image: row.get(11),
                following: row.get(12),
            },
        }
    }
}

impl From<Row> for CommentInfo {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0) ,
            created_at: row.get(1),
            updated_at: row.get(2),
            body: row.get(3),
            author: ProfileInfo {
                username:row.get(4),
                bio: row.get(5),
                image: row.get(6),
                following: row.get(7),
            },
        }
    }
}


impl From<Row> for ArticleInfo {
    fn from(row: Row) -> Self {
        Self {
            slug: row.get(0),
            title: row.get(1),
            description: row.get(2),
            body: row.get(3),
            tag_list: row.get(4),
            created_at: row.get(5),
            updated_at: row.get(6),
            favorited: row.get(7),
            favorites_count: row.get(8),
            author: row.get(9),

        }
    }
}


impl From<Row> for TagListInfo {
    fn from(row: Row) -> Self {
        Self {
            tags: row.get(0),
        }
    }
}