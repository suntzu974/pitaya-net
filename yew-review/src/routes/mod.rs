//! Routes by yew_router

pub mod home;

use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::reviews::Reviews;

use self::home::Home;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
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
        AppRoute::NotFound => html! { "Page not found" },
    }
}
