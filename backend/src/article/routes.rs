use actix_web::{get, post, put , web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use super::models::{ArticleCreateUpdateInfoWrapper,ArticleInfoWithProfileWrapper,ArticleListInfo,
    TagListInfo,Query,ArticleInfoWithProfile,CommentInfoWrapper,CommentListInfo,CommentCreateInfoWrapper};
use itertools::Itertools;
use actix_web_httpauth::extractors::bearer::BearerAuth;

#[post("/articles")]
pub async fn add_article(
    author:BearerAuth,
    article: web::Json<ArticleCreateUpdateInfoWrapper>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.unwrap();
    let article_info: ArticleCreateUpdateInfoWrapper = article.into_inner();
    let new_article = ArticleInfoWithProfile::add_article(&client, article_info.article,
        author.token().to_string()).await.unwrap();
    Ok(HttpResponse::Ok().json(ArticleInfoWithProfileWrapper{ article: new_article,}))
}
#[put("/articles/{slug}")]
pub async fn update_article(
    slug: web::Path<String>,
    article: web::Json<ArticleCreateUpdateInfoWrapper>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.unwrap();
    let article_info: ArticleCreateUpdateInfoWrapper = article.into_inner();
    let updated_article = ArticleInfoWithProfile::update_article(slug.to_string(),&client, article_info.article,
        ).await.unwrap();
    Ok(HttpResponse::Ok().json(ArticleInfoWithProfileWrapper{ article: updated_article,}))
}

#[get("/articles/feed")]
pub async fn get_articles(
    author:BearerAuth,
    info:web::Query<Query>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {

    let client: Client = db_pool.get().await.unwrap();
    let articles = ArticleInfoWithProfile::get_feed_articles(author.token().to_string(),&info,&client).await?;
    Ok(HttpResponse::Ok().json(ArticleListInfo{ 
                            articles: articles.clone(),
                            articles_count: articles.len() as u32,}))
}

#[get("/articles")]
pub async fn get_all_articles(
    info:web::Query<Query>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {

    let client: Client = db_pool.get().await.unwrap();
    let articles = ArticleInfoWithProfile::get_articles(&info,&client).await?;
    Ok(HttpResponse::Ok().json(ArticleListInfo{ 
                            articles: articles.clone(),
                            articles_count: articles.len() as u32,}))
}

#[get("/articles/{slug}")]
pub async fn get_article(
    slug:web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {

    let client: Client = db_pool.get().await.unwrap();
    let article = ArticleInfoWithProfile::get_article(slug.into_inner(),&client).await?;
    Ok(HttpResponse::Ok().json(ArticleInfoWithProfileWrapper{ article: article,}))
}
#[get("/articles/{slug}/comments")]
pub async fn get_comments_for_article(
    slug:web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {

    let client: Client = db_pool.get().await.unwrap();
    let comments = ArticleInfoWithProfile::get_comments(slug.into_inner(),&client).await?;
    Ok(HttpResponse::Ok().json(CommentListInfo{ comments: comments,}))
}
#[post("/articles/{slug}/comments")]
pub async fn create_comment_for_article(
    author:BearerAuth,
    slug: web::Path<String>,
    comment: web::Json<CommentCreateInfoWrapper>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.unwrap();
    let comment_info: CommentCreateInfoWrapper = comment.into_inner();
    let new_comment = ArticleInfoWithProfile::add_comment(&client, comment_info.comment,
        slug.into_inner(),author.token().to_string()).await.unwrap();
    Ok(HttpResponse::Ok().json(CommentInfoWrapper{ comment: new_comment,}))
}

#[get("/tags")]
pub async fn get_tags(
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.unwrap();
    let tags:Vec<TagListInfo> = ArticleInfoWithProfile::get_tags(&client).await?;
    let mut alltags:Vec<String> = Vec::new();
    for val in tags.iter() {
        for tag in val.tags.iter() {
            alltags.push(tag.to_string());
        }
    }
    let v:Vec<String> = alltags.into_iter().unique().collect();
    let tags = TagListInfo {tags:v,};
    Ok(HttpResponse::Ok().json(tags))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add_article);
    cfg.service(update_article);
    cfg.service(get_articles);
    cfg.service(get_article);
    cfg.service(get_all_articles);
    cfg.service(get_tags);
    cfg.service(get_comments_for_article);
    cfg.service(create_comment_for_article);
}