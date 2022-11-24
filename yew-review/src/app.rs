
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{switch, AppRoute};
use crate::components::footer::Footer;
use crate::components::header::Header;

/// The root app component
#[function_component(PitayaApp)]
pub fn app() -> Html {
    html! {
            <BrowserRouter>
                <Header />
                <Switch<AppRoute> render={Switch::render(switch)} />
                <Footer />
            </BrowserRouter>
    }
}
