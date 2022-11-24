use yew::prelude::*;

use crate::components::reviews::{Reviews};


#[derive(PartialEq, Clone)]
pub enum Tab {
    All,
    Feed,
    Tag,
}

/// Main content with tabs of article list for home page
#[function_component(MainView)]
pub fn main_view() -> Html {

    html! {
        <div class="col-md-9 col-xs-12">
            <Reviews />
        </div>
    }
}
