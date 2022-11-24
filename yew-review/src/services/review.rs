use crate::services::requests::{limit, request_get, request_post, request_put};
use crate::models::review::*;
use crate::error::Error;
/// Get all reviews
pub async fn get_all() -> Result<ReviewListInfo, Error> {
    request_get::<ReviewListInfo>("/reviews".to_string()).await
}
/// Get an review
pub async fn get(id: i32) -> Result<ReviewInfoWrapper, Error> {
    request_get::<ReviewInfoWrapper>(format!("/reviews/{}", id)).await
}

/// Update an review
pub async fn update(
    id: i32,
    review: ReviewCreateUpdateInfoWrapper,
) -> Result<ReviewInfoWrapper, Error> {
    request_put::<ReviewCreateUpdateInfoWrapper, ReviewInfoWrapper>(
        format!("/reviews/{}", id),
        review,
    )
    .await
}

/// Create an review
pub async fn create(review: ReviewCreateUpdateInfoWrapper) -> Result<ReviewInfoWrapper, Error> {
    request_post::<ReviewCreateUpdateInfoWrapper, ReviewInfoWrapper>(
        "/reviews".to_string(),
        review,
    )
    .await
}

