use yew::prelude::*;
use crate::models::review::ReviewInfo;


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub review: ReviewInfo,
}

/// Single article preview component used by article list.
#[function_component(ReviewPreview)]
pub fn review_preview(props: &Props) -> Html {
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
