use argon2::{Config as argonConfig};
use crate::errors::ServiceError;
use config::ConfigError;
use serde::{Deserialize };

#[derive(Debug, Deserialize,Clone)]
pub struct Config {
    pub pg_host:Option<String>,
    pub pg_port:Option<u16>,
    pub pg_dbname:Option<String>,
    pub pg_user:Option<String>,
    pub pg_password:Option<String>,
    pub backend_host:String,
    pub backend_port:u16,
    pub ssl_crt:String,
    pub ssl_key:String,
    pub web_host:String,
    pub pictures_originals:String,
    pub pictures_thumbnails:String,
    pub pictures_web:String,
}

impl Config {
    #[cfg(target_os = "windows")]
    pub fn from_env() -> Result<Self, ConfigError> {
          config::Config::builder()
              .add_source(config::File::with_name("c:\\pitaya\\pitaya.toml"))
              .build()
              .unwrap()
              .try_deserialize()
      }
      #[cfg(not(target_os = "windows"))]
      pub fn from_env() -> Result<Self, ConfigError> {
            config::Config::builder()
                .add_source(config::File::with_name("/usr/local/var/pitaya.toml"))
                .build()
                .unwrap()
                .try_deserialize()
    }
}

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
}

const SALT: &'static [u8] = b"supersecuresalt";

// WARNING THIS IS ONLY FOR DEMO PLEASE DO MORE RESEARCH FOR PRODUCTION USE
pub async fn hash_password(password: &str) -> Result<String, ServiceError> {
    let config = argonConfig {
        secret: SECRET_KEY.as_bytes(),
        ..Default::default()
    };
     argon2::hash_encoded(password.as_bytes(), &SALT, &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

/*pub async fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
     argon2::verify_encoded_ext(hash, password.as_bytes(), SECRET_KEY.as_bytes(), &[]).map_err(
        |err| {
            dbg!(err);
            ServiceError::Unauthorized
        },
    )
}*/
extern crate image;

use image::ImageFormat;
pub async fn resize_for_thumbnail(filename: String) -> String {
    let pictures = Config::from_env().unwrap();

    let mut filepath = format!("{}/{}", pictures.pictures_originals, sanitize_filename::sanitize(&filename));
    let img = image::open(filepath).unwrap();
    let scaled = img.thumbnail(400, 400);


    filepath = format!("{}/{}", pictures.pictures_thumbnails, sanitize_filename::sanitize(&filename));
    let smallpicture = format!("{}", sanitize_filename::sanitize(&filename));
    let mut output = std::fs::File::create(filepath.clone()).unwrap();
    scaled.write_to(&mut output, ImageFormat::Jpeg).unwrap();
    let urlpath = format!("{}://{}:{}/thumbnail/{}","https",pictures.web_host,pictures.backend_port,smallpicture,);
    return urlpath.clone();
}
pub async fn resize_for_web(filename: String) -> String {
    let pictures = Config::from_env().unwrap();


    let mut filepath = format!("{}/{}", pictures.pictures_originals, sanitize_filename::sanitize(&filename));

    let img = image::open(filepath).unwrap();
    let scaled = img.thumbnail(1024, 768);


    filepath = format!("{}/{}", pictures.pictures_web, sanitize_filename::sanitize(&filename));
    let smallpicture = format!("{}", sanitize_filename::sanitize(&filename));
    let mut output = std::fs::File::create(filepath.clone()).unwrap();
    scaled.write_to(&mut output, ImageFormat::Jpeg).unwrap();
    let urlpath = format!("{}://{}:{}/web/{}","https",pictures.web_host,pictures.backend_port,smallpicture,);
    return urlpath.clone();
}
