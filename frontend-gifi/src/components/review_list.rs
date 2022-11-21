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
                ReviewicleListFilter::ByAuthor(author) => by_author(author, *current_page).await,
                ArticleListFilter::ByTag(tag) => by_tag(tag, *current_page).await,
                ArticleListFilter::FavoritedBy(author) => favorited_by(author, *current_page).await,
                ArticleListFilter::Feed => feed().await,
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
        let article_list = article_list.clone();
        use_effect_with_deps(
            move |_| {
                article_list.run();
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

    if let Some(article_list) = &article_list.data {
        if !article_list.articles.is_empty() {
            html! {
                <>
                    {for article_list.articles.iter().map(|article| {
                        html! { <ArticlePreview article={article.clone()} /> }
                    })}
                    <ListPagination
                        total_count={article_list.articles_count}
                        current_page={*current_page}
                        callback={callback} />
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
