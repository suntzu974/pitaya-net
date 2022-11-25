mod banner;
mod main_view;

use yew::prelude::*;

use banner::Banner;
use main_view::MainView;

/// Home page with an article list and a tag list.
#[function_component(Home)]
pub fn home() -> Html {

    html! {
        <div class="home-page">
            <Banner />
            <div class="container page">
                <div class="card-column">
                    <MainView />
                </div>
            </div>
        </div>
    }
}
