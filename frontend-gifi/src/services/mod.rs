//! Api requests via yew FetchService

pub mod review;
pub mod requests;

pub use requests::{
    get_token, limit, request_delete, request_get, request_post, request_put, set_token,
};
