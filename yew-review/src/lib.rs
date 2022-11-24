#![recursion_limit = "256"]
mod services;
mod models;
mod types;
mod error;

use gloo_net::http::Request;
use wasm_bindgen::prelude::*;
use yew_router::prelude::*;
use yew_hooks::use_async;
use crate::models::review::{ReviewListInfo};

use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::services::review::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/reviews")]
    List,
//    #[at("/reviews/{id}")]
//    Detail{slug: i32},

}
fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Hello Frontend" }</h1> },
        Route::List => html! {<List/> },
//        Route::Detail {slug} => html! {<Detail slug = {slug.clone()}/>},
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }

}

/// Filters for article list

#[function_component(List)]
pub fn review_list() -> Html {
    let current_page = use_state(|| 0u32);

    let data = use_state(|| None);
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("http://localhost:3011/reviews?limit=100&offset=0").send().await.unwrap();
                    let result = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.text().await.map_err(|err| err.to_string())
                        }
                    };
                    data.set(Some(result));
                });
            }

            || {}
        });
    }

    match data.as_ref() {
        None => {
            html! {
                <div>{"No server response"}</div>
            }
        }
        Some(Ok(data)) => {
            html! {
                <div>{data}</div>
            }
        }
        Some(Err(err)) => {
            html! {
                <div>{"Error requesting data from server: "}{err}</div>
            }
        }
    }

}
/*
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub slug: i32,
}
#[function_component(Detail)]
fn detail(props: &Props) -> Html {

    let review = {
        let slug = props.slug.clone();

        use_async_with_options(
            async move { get(slug).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    let review = use_state(|| review.clone());
 
    html! {
        <div class="article-preview">
            <h1>
                { &review.title }
            </h1>
            <p>{ &review.description }</p>
            <div class="article-meta">
                <img alt="image" src={review.thumbnail.clone()} />
            </div>
        </div>
    }
}
*/


#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}