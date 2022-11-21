use calamine::{DataType, Range, Reader, open_workbook,Xlsx};
use actix_multipart_extract::{ Multipart};
use crate::container::{Container,InputContainer,ContainerForm,FileDownloaded};
use actix_web::{delete,get,post,put,web, Error, HttpResponse};
use std::fs;
use std::path::Path;
use deadpool_postgres::{Client, Pool};
use crate::utils::utils::Config;

#[get("/containers")]
async fn find_all(
    db: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
    let client: Client = db.get().await.unwrap();
    let containers = Container::get_containers(&client).await.unwrap();
    Ok(HttpResponse::Ok().json(containers))
}

#[get("/containers/{container}")]
async fn find_by_id(
    db: web::Data<Pool>,
    container_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let client: Client = db.get().await.unwrap();
    let container = Container::get_container_by_id(container_id.into_inner(),&client).await.unwrap();
    Ok(HttpResponse::Ok().json(container))
}

#[post("/containers")]
pub async fn create(
    db: web::Data<Pool>,
    item: web::Json<InputContainer>,
) -> Result<HttpResponse, Error> {
    let client: Client = db.get().await.unwrap();
    let input_container:InputContainer = item.into_inner();
    let new_container = Container::insert(&client,input_container ).await.unwrap();
    Ok(HttpResponse::Ok().json(new_container))
}

#[delete("/containers/{container}")]
pub async fn remove(
    db: web::Data<Pool>,
    container_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let client: Client = db.get().await.unwrap();
    Container::delete_container(container_id.into_inner(),&client).await.unwrap();
    Ok(HttpResponse::Ok().finish())
}

#[put("/containers/{container}")]
pub async fn put(
    db: web::Data<Pool>,
    container_id: web::Path<i32>,
    item: web::Json<InputContainer>,
) -> Result<HttpResponse, Error> {
    let client: Client = db.get().await.unwrap();
    let updated_container:InputContainer = item.into_inner();
    let result = Container::update(container_id.into_inner(),updated_container,&client).await.unwrap();
    Ok(HttpResponse::Ok().json(result))
}
#[post("/uploadxls")]
pub async fn upload(
            db: web::Data<Pool>,
            container_form: Multipart<ContainerForm>) -> Result<HttpResponse, Error> {
    let file_path = Config::from_env().unwrap();
    fs::create_dir_all(file_path.files_directory.to_string())?;

    let _client: Client = db.get().await.unwrap();

    let path_file = format!("{}/{}",file_path.files_directory,&container_form.file.name);
    fs::write( Path::new(&path_file), &container_form.file.bytes).unwrap();
    let _urlpath = format!("{}://{}:{}/download/{}","https",file_path.web_host,file_path.backend_port,
    sanitize_filename::sanitize(&container_form.file.name));
    let mut xl: Xlsx<_> = open_workbook(&path_file).unwrap();
    let range = xl.worksheet_range_at(0).unwrap().unwrap();
    let mut containers = Vec::new();
    range.rows().into_iter().skip(1).for_each(|row| {
    
        let downloaded_container = InputContainer {
            container: row[0].to_string(),
            facture: row[1].to_string().parse::<i32>().unwrap(),
            article:row[2].to_string(),
            designation:row[3].to_string(),
            poids_colis:row[4].to_string().parse::<f32>().unwrap(),
            poids_commande:row[5].to_string().parse::<f32>().unwrap(),
            volume:row[6].to_string().parse::<f32>().unwrap(),
            pcb:row[7].to_string().parse::<i32>().unwrap(),
            spcb:row[8].to_string().parse::<i32>().unwrap(),
            pv:row[9].to_string().parse::<f32>().unwrap(),
            pvconseil:row[10].to_string().parse::<f32>().unwrap(),
            qte:row[11].to_string().parse::<i32>().unwrap(),
            montant:row[12].to_string().parse::<f32>().unwrap(),
            date:row[13].to_string().parse::<i32>().unwrap(),
            palette:row[14].to_string(),
            origine:row[15].to_string(),
            ean:row[16].to_string(),
            theme:row[17].to_string(),
            codedouanier:row[18].to_string(),
            commande:row[19].to_string(),
            libunivers:row[20].to_string(),
            univers:row[21].to_string(),
            libfamille:row[22].to_string(),
            famille:row[23].to_string(),
            libsfamille:row[24].to_string(),
            sfamille:row[25].to_string()
        };
        containers.push(downloaded_container);
    }
    );
    Ok(HttpResponse::Ok().json(containers))
}

#[get("/downloadxls/{container}")]
pub async fn download(info: web::Path<FileDownloaded>) -> HttpResponse {
    let file_path = Config::from_env().unwrap();

    let path = format!("{}/{}", file_path.files_directory, info.name.to_string());
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

#[delete("/removexls/{container}")]
pub async fn remove_download(info: web::Path<FileDownloaded>) -> HttpResponse {
    let file_path = Config::from_env().unwrap();
    let path = format!("{}/{}", file_path.files_directory, info.name.to_string());
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


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find_by_id);
    cfg.service(create);
    cfg.service(put);
    cfg.service(remove);
    cfg.service(upload);
    cfg.service(download);
    cfg.service(remove);
}