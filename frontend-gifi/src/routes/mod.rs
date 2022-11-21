//! Routes by yew_router

pub mod article;
pub mod editor;
pub mod home;
pub mod login;
pub mod profile;
pub mod register;
pub mod settings;

use yew::prelude::*;
use yew_router::prelude::*;

use review::Review;
use home::Home;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/review")]
    ReviewList,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::ReviewList => html! {<ReviewList />},
        AppRoute::Home => html! {<Home />},
        AppRoute::NotFound => html! { "Page not found" },
    }
}
