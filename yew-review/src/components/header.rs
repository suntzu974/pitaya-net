use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[function_component(Header)]
pub fn header() -> Html {

    html! {
        <nav class="navbar navbar-light">
            <div class="container">
                <Link<AppRoute> to={AppRoute::Home} classes="navbar-brand">
                    { "moinlela" }
                </Link<AppRoute>>
                {
                    logged_out_view()
                }
            </div>
        </nav>
    }
}

fn logged_out_view() -> Html {
    html! {
        <ul class="nav navbar-nav pull-xs-right">
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                    { "Home" }
                </Link<AppRoute>>
            </li>
            <li class="nav-item">
                <Link<AppRoute> to={AppRoute::ReviewList} classes="nav-link">
                    { "Reviews" }
                </Link<AppRoute>>
            </li>
        </ul>
    }
}
