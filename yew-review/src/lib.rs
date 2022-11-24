#![recursion_limit = "256"]
mod services;
mod models;
mod types;
mod error;

use gloo_net::http::Request;
use wasm_bindgen::prelude::*;
use yew_router::prelude::*;
use yew_hooks::use_async;
use crate::models::review::{ReviewListInfo,ReviewInfo};

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
    #[at("/list")]
    List1,
//    #[at("/reviews/{id}")]
//    Detail{slug: i32},

}
fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Hello Frontend" }</h1> },
        Route::List => html! {<List/> },
        Route::List1 => html! {<List1/> },
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

#[function_component(List1)]
pub fn review_list() -> Html {
    let current_page = use_state(|| 0u32);

    let data = use_state(|| None);
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("http://localhost:3011/reviews?limit=100&offset=0")
                                .send()
                                .await
                                .unwrap();
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

#[function_component(List)]
pub fn reviews() -> Html {
    let current_page = use_state(|| 0u32);
    let review_list = {
        let current_page = current_page.clone();
        use_async(async move { all(*current_page).await })
    };

    if let Some(review_list) = &review_list.data {
        if !review_list.reviews.is_empty() {
            html! {
                <>
                    {for review_list.reviews.iter().map(|review| {
                        html! { <ReviewPreview review={review.clone()} /> }
                    })}
                </>
            }
        } else {
            html! {
                <div class="article-preview">{ "No articles are here... yet." }</div>
            }
        }
    } else {
        html! {
            <div class="article-preview">{ "Loading..." }</div>
        }
    }

}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub review: ReviewInfo,
}

/// Single article preview component used by article list.
#[function_component(ReviewPreview)]
pub fn article_preview(props: &Props) -> Html {
    let review = use_state(|| props.review.clone());

    html! {
        <div class="article-preview">
            <div class="article-meta">
                <img alt="image" src={review.thumbnail.clone()} />
            </div>
            <p>{ &review.description }</p>
        </div>
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
