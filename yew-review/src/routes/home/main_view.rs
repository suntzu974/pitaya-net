use yew::prelude::*;

use crate::components::reviews::{Reviews};


/// Main content with tabs of article list for home page
#[function_component(MainView)]
pub fn main_view() -> Html {

    html! {
        <div class="container">
            <Reviews />
        </div>
    }
}
