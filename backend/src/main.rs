use actix_web::{middleware, web, App, Result, HttpServer,http::header};
use actix_web_httpauth::extractors::bearer::{self};
use dotenv::dotenv;
use actix_cors::Cors;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::{File};
use std::io::BufReader;
use deadpool_postgres::{Pool, Runtime};
use deadpool_postgres::{ManagerConfig, RecyclingMethod};
use crate::utils::utils::{Config};

mod utils;
mod errors;
mod user;
mod article;
mod review;
mod comment;
mod container;


fn get_db_config() -> deadpool_postgres::Config {
    let config_db = Config::from_env().unwrap();
    let mut config = deadpool_postgres::Config::new();

    config.user = config_db.pg_user ; //Some(env_parse("DB_USER", "postgres".into()));
    config.password = config_db.pg_password; // Some(env_parse("DB_PASSWORD", "password".into()));
    config.dbname = config_db.pg_dbname; //Some(env_parse("DB_NAME", "postgres".into()));
    config.host = config_db.pg_host; //Some(env_parse("DB_HOSTNAME", "172.17.0.2".into()));
    config.port = config_db.pg_port;
    config.manager =
       Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    config
}
pub fn create_pool() -> Result<Pool, String> {
    Ok(get_db_config().create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls).map_err(|err| err.to_string())?)
}

#[cfg(debug_assertions)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
 
    env_logger::init();
    dotenv().ok();

    let config_db = Config::from_env().unwrap();
    let pool:Pool = create_pool().unwrap();
    println!("Starting http server: {}:{} ",config_db.backend_host.clone(),config_db.backend_port.clone());

    HttpServer::new(move || {

        let cors = Cors::default()
        .allowed_origin("https://www.goyav.re:8443")
        .allowed_origin("https://www.goyav.re:443")
        .allowed_origin("https://www.goyav.re:3011")
        .allowed_origin("https://www.goyav.re:3010")
        .allowed_origin("https://www.goyav.re")
        .allowed_origin("http://localhost")
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET","PUT","POST","DELETE"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600);
        App::new()
            .wrap(cors)
            // enable logger
            .wrap(middleware::Logger::default())
            // register simple handler, handle all methods
            .app_data(web::Data::new(pool.clone()))
            .app_data(bearer::Config::default)
            .configure(user::init_routes)
            .configure(article::init_routes)
            .configure(review::init_routes)
            .configure(comment::init_routes)
            .configure(container::init_routes)
    })
    .bind(format!("{}:{}" ,config_db.backend_host,config_db.backend_port))?
    .run()
    .await
}
#[cfg(not(debug_assertions))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
 
    env_logger::init();
    dotenv().ok();

    let config_db = Config::from_env().unwrap();
    let pool:Pool = create_pool().unwrap();

    // load ssl keys
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();
    let cert_file = &mut BufReader::new(File::open(config_db.ssl_crt.clone()).unwrap());
    let key_file = &mut BufReader::new(File::open(config_db.ssl_key.clone()).unwrap());
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }
    let config = config.with_single_cert(cert_chain, keys.remove(0)).unwrap();
    // Read Settings;
    println!("Starting https server: {}:{} ",config_db.backend_host.clone(),config_db.backend_port.clone());
    HttpServer::new(move || {

        let cors = Cors::default()
        .allowed_origin("https://www.goyav.re:8443")
        .allowed_origin("https://www.goyav.re:443")
        .allowed_origin("https://www.goyav.re:3011")
        .allowed_origin("https://www.goyav.re:3010")
        .allowed_origin("https://www.goyav.re")
        .allowed_origin("http://localhost")
        .allowed_origin("http://localhost:8080")
        .allowed_methods(vec!["GET","PUT","POST","DELETE"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600);
        App::new()
            .wrap(cors)
            // enable logger
            .wrap(middleware::Logger::default())
            // register simple handler, handle all methods
            .app_data(web::Data::new(pool.clone()))
            .app_data(bearer::Config::default)
            .configure(user::init_routes)
            .configure(article::init_routes)
            .configure(review::init_routes)
            .configure(comment::init_routes)
            .configure(container::init_routes)
    })
    .bind_rustls(format!("{}:{}" ,config_db.backend_host,config_db.backend_port,), config)?
    .run()
    .await
}