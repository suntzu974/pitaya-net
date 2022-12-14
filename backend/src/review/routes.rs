use actix_multipart_extract::{ Multipart};
use crate::review::{Review,InputReview,ReviewForm,FileDownloaded,ReviewListInfo,Query};
use actix_web::{delete,get,post,put,web, Error, HttpResponse};
use crate::utils::utils::{resize_for_thumbnail,resize_for_web,Config};
use std::fs;
use std::path::Path;
use deadpool_postgres::{Client, Pool};

#[get("/reviews")]
async fn find_all(
    db: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
    let client: Client = db.get().await.unwrap();
    let reviews = Review::get_reviews(&client).await.unwrap();
    Ok(HttpResponse::Ok().json(ReviewListInfo { reviews: reviews.clone(),
                                                reviews_count: reviews.len() as u32 }))
}

// Handler for GET /users/{id}
#[get("/reviews/{id}")]
async fn find_by_id(
    db: web::Data<Pool>,
    review_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let client: Client = db.get().await.unwrap();
    let review = Review::get_review_by_id(review_id.into_inner(),&client).await.unwrap();
    Ok(HttpResponse::Ok().json(review))
}

#[post("/reviews")]
pub async fn create(
    db: web::Data<Pool>,
    item: web::Json<InputReview>,
) -> Result<HttpResponse, Error> {
    let client: Client = db.get().await.unwrap();
    let input_review:InputReview = item.into_inner();
    let new_review = Review::insert(&client,input_review ).await.unwrap();
    Ok(HttpResponse::Ok().json(new_review))
}

// Handler for DELETE /reviews/{id}
#[delete("/reviews/{id}")]
pub async fn remove(
    db: web::Data<Pool>,
    review_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let client: Client = db.get().await.unwrap();
    Review::delete_review(review_id.into_inner(),&client).await.unwrap();
    Ok(HttpResponse::Ok().finish())
}

#[put("/reviews/{id}")]
pub async fn put(
    db: web::Data<Pool>,
    review_id: web::Path<i32>,
    item: web::Json<InputReview>,
) -> Result<HttpResponse, Error> {
    let client: Client = db.get().await.unwrap();
    let updated_review:InputReview = item.into_inner();
    let result = Review::update(review_id.into_inner(),updated_review,&client).await.unwrap();
    Ok(HttpResponse::Ok().json(result))
}
#[post("/upload")]
pub async fn upload(
            db: web::Data<Pool>,
            review_form: Multipart<ReviewForm>) -> Result<HttpResponse, Error> {
    let pictures = Config::from_env().unwrap();
    fs::create_dir_all(pictures.pictures_originals.to_string())?;
    fs::create_dir_all(pictures.pictures_thumbnails.to_string())?;
    fs::create_dir_all(pictures.pictures_web.to_string())?;

    let path_originals = format!("{}/{}",pictures.pictures_originals,&review_form.picture.name);
    fs::write( Path::new(&path_originals), &review_form.picture.bytes).unwrap();
    #[cfg(debug_assertions)]
    let urlpath = format!("{}://{}:{}/download/{}","http",pictures.web_host,pictures.backend_port,sanitize_filename::sanitize(&review_form.picture.name));
    #[cfg(not(debug_assertions))]
    let urlpath = format!("{}://{}:{}/download/{}","https",pictures.web_host,pictures.backend_port,sanitize_filename::sanitize(&review_form.picture.name));

    
  
    let downloaded_review = InputReview {
        title: review_form.title.clone(),
        description: review_form.description.clone(),
        original: urlpath,
        thumbnail: resize_for_thumbnail(review_form.picture.name.clone()).await,
        web: resize_for_web(review_form.picture.name.clone()).await,
        deleted: false,
    };
    let client: Client = db.get().await.unwrap();
    let new_review = Review::insert(&client,downloaded_review ).await?;
    Ok(HttpResponse::Ok().json(new_review))
}

#[get("/download/{name}")]
pub async fn download(info: web::Path<FileDownloaded>) -> HttpResponse {
    let pictures = Config::from_env().unwrap();

    let path = format!("{}/{}", pictures.pictures_originals, info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&FileDownloaded {
            name: info.name.to_string(),
        });
    }
    let data = fs::read(path).unwrap();
    HttpResponse::Ok()
        .append_header(("Content-Disposition", format!("form-data; filename={}", info.name.to_string())))
        .body(data)
}
#[get("/thumbnail/{name}")]
pub async fn thumbnail(info: web::Path<FileDownloaded>) -> HttpResponse {
    let pictures = Config::from_env().unwrap();
    let path = format!("{}/{}", pictures.pictures_thumbnails, info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&FileDownloaded {
            name: info.name.to_string(),
        });
    }
    let data = fs::read(path).unwrap();
    HttpResponse::Ok()
        .append_header(("Content-Disposition", format!("form-data; filename={}", info.name.to_string())))
        .body(data)
}
#[get("/web/{name}")]
pub async fn webpicture(info: web::Path<FileDownloaded>) -> HttpResponse {
    let pictures = Config::from_env().unwrap();
    let path = format!("{}/{}", pictures.pictures_web, info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&FileDownloaded {
            name: info.name.to_string(),
        });
    }
    let data = fs::read(path).unwrap();
    HttpResponse::Ok()
        .append_header(("Content-Disposition", format!("form-data; filename={}", info.name.to_string())))
        .body(data)
}

#[delete("/removeOriginal/{name}")]
pub async fn remove_download(info: web::Path<FileDownloaded>) -> HttpResponse {
    let pictures = Config::from_env().unwrap();
    let path = format!("{}/{}", pictures.pictures_originals, info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&FileDownloaded {
            name: info.name.to_string(),
        });
    }
    fs::remove_file(path).unwrap();
    HttpResponse::Ok()
        .append_header(("Content-Disposition", format!("form-data; filename={}", info.name.to_string())))
        .body("File deleted")
}

#[delete("/removeThumbnail/{name}")]
pub async fn remove_thumbnail(info: web::Path<FileDownloaded>) -> HttpResponse {
    let pictures = Config::from_env().unwrap();
    let path = format!("{}/{}", pictures.pictures_thumbnails, info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&FileDownloaded {
            name: info.name.to_string(),
        });
    }
    fs::remove_file(path).unwrap();
    HttpResponse::Ok()
        .append_header(("Content-Disposition", format!("form-data; filename={}", info.name.to_string())))
        .body("file deleted")
}
#[delete("/removeWeb/{name}")]
pub async fn remove_web(info: web::Path<FileDownloaded>) -> HttpResponse {
    let pictures = Config::from_env().unwrap();
    let path = format!("{}/{}", pictures.pictures_web, info.name.to_string());
    if !Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&FileDownloaded {
            name: info.name.to_string(),
        });
    }
    fs::remove_file(path).unwrap();
    HttpResponse::Ok()
        .append_header(("Content-Disposition", format!("form-data; filename={}", info.name.to_string())))
        .body("file deleted")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_by_id);
    cfg.service(create);
    cfg.service(put);
    cfg.service(remove);
    cfg.service(upload);
    cfg.service(download);
    cfg.service(thumbnail);
    cfg.service(webpicture);
    cfg.service(remove_download);
    cfg.service(remove_thumbnail);
    cfg.service(remove_web);
}