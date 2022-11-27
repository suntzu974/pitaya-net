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
        <div class="card" style="width:230px;padding-right: 10x;padding-left: 10px">
                <img class="img-square-wrapper" alt="image" src={review.thumbnail.clone()} width={"225"} height={"150"} />
                <div class="card-body">
                    <h4 class="card-title" >{&review.title} </h4>
                    <p class="card-text">{ &review.description }</p>
                </div>
        </div>
    }
}
