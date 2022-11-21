use yew::prelude::*;
use yew_hooks::use_async;

use super::review_preview::ReviewPreview;
use crate::services::reviews::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub filter: ReviewListFilter,
}

/// Filters for article list
#[derive(Clone, Debug, PartialEq)]
pub enum ReviewListFilter {
    All,
    Deleted(bool),
}

/// List of articles component
#[function_component(ReviewList)]
pub fn review_list(props: &Props) -> Html {
    let current_page = use_state(|| 0u32);
    let review_list = {
        let filter = props.filter.clone();
        let current_page = current_page.clone();

        use_async(async move {
            match filter {
                ReviewListFilter::All => all(*current_page).await,
            }
        })
    };

    {
        let current_page = current_page.clone();
        use_effect_with_deps(
            move |_| {
                // Reset to first page
                current_page.set(0);
                || ()
            },
            props.filter.clone(),
        );
    }

    {
        let review_list = review_list.clone();
        use_effect_with_deps(
            move |_| {
                review_list.run();
                || ()
            },
            (props.filter.clone(), *current_page),
        );
    }

    let callback = {
        let current_page = current_page.clone();
        Callback::from(move |page| {
            current_page.set(page);
        })
    };

    if let Some(review_list) = &review_list.data {
        if !review_list.review.is_empty() {
            html! {
                <>
                    {for review_list.articles.iter().map(|review| {
                        html! { <ReviewPreview review={review.clone()} /> }
                    })}
                    <ListPagination
                        total_count={review_list.reviews_count}
                        current_page={*current_page}
                        callback={callback} />
                </>
            }
        } else {
            html! {
                <div class="article-preview">{ "No reviews are here... yet." }</div>
            }
        }
    } else {
        html! {
            <div class="article-preview">{ "Loading..." }</div>
        }
    }
}
