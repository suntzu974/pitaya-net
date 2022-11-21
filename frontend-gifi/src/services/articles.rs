use super::{limit, request_delete, request_get, request_post, request_put};
use crate::error::Error;
use crate::types::*;

/// Get all articles
pub async fn all(page: u32) -> Result<ReviewListInfo, Error> {
    request_get::<ReviewListInfo>(format!("/reviews?{}", limit(10, page))).await
}
