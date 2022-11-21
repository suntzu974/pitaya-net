use yew::{function_component, html};
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="container">
                <Link<AppRoute> to={AppRoute::Home} classes="logo-font">{ "moinlela" }</Link<AppRoute>>
                <span class="attribution">
                    { "© 2019. An interactive learning project from" }
                    <a href="https://www.goyav.re"> { "Goyav" } </a>
                    { ". Code licensed under MIT." }
                </span>
            </div>
        </footer>
    }
}
