use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use crate::components::review_preview::ReviewPreview;

use crate::services::review::*;

/// A tag list component with a callback to notify that some tag is clicked.
#[function_component(Reviews)]
pub fn reviews() -> Html {
    let review_list = use_async_with_options(
        async move { get_all().await },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(review_list) = &review_list.data {
        html! {
            <div class="container">
                {for review_list.reviews.iter().map(|review| {
                    html! {
                        <ReviewPreview review={review.clone()} /> 
                    }
                })}
            </div>
        }
    } else {
        html! {
            <div>{ "Loading Reviews..." }</div>
        }
    }
}
