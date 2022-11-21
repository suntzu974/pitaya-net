//! Common types

mod articles;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use articles::{
    ArticleCreateUpdateInfo, ArticleCreateUpdateInfoWrapper, ArticleInfo, ArticleInfoWrapper,
    ArticleListInfo,
};

/// Conduit api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub type DeleteWrapper = HashMap<(), ()>;
