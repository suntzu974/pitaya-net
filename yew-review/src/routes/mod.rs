//! Routes by yew_router

pub mod home;

use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::reviews::Reviews;
use crate::components::file_upload::FileUpload;

use self::home::Home;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/upload")]
    FileUpload,
    #[at("/reviews")]
    ReviewList,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Home => html!(<Home/>),
        AppRoute::ReviewList => html! {<Reviews />},
        AppRoute::FileUpload => html! {<FileUpload />},
        AppRoute::NotFound => html! { "Page not found" },
    }
}
