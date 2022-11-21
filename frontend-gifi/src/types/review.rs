use chrono::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReviewInfo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub original: String,
    #[serde(rename="url")]
    pub thumbnail: String,
    pub web: String,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReviewInfoWrapper {
    pub review: ReviewInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewListInfo {
    pub reviews: Vec<ReviewInfo>,
    pub reviews_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReviewCreateUpdateInfo {
    pub title: String,
    pub description: String,
    pub original: String,
    pub thumbnail: String,
    pub web: String,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReviewCreateUpdateInfoWrapper {
    pub review: ReviewCreateUpdateInfo,
}
