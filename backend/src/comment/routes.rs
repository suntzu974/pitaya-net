use actix_web::{get, post, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use super::models::{Comment,CommentWrapper,CommentCreateWrapper,Counted,CommentQuery};
use actix_web_httpauth::extractors::bearer::BearerAuth;

#[post("/comments/{slug}")]
pub async fn add_comment(
    author:BearerAuth,
    slug: web::Path<String>,
    comment: web::Json<CommentCreateWrapper>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.unwrap();
    let comment_info: CommentCreateWrapper = comment.into_inner();
    let new_comment = Comment::add_comment(&client, comment_info.comment,
        slug.into_inner(),author.token().to_string()).await.unwrap();
    Ok(HttpResponse::Ok().json(CommentWrapper{ comment: new_comment,}))
}

#[get("/comments/{slug}")]
pub async fn get_comments(
    info: web::Query<CommentQuery>,
    slug: web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {

    let client: Client = db_pool.get().await.unwrap();
    let comments = Comment::get_comments(&info,slug.into_inner(),&client).await?;
    let counted = Counted {
        total: comments.len() as u32,
        results: comments,
    };
    Ok(HttpResponse::Ok().json(counted))
}



pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(add_comment);
    cfg.service(get_comments);
}