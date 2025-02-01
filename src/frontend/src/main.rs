use router::{switch, Route};
use yew_router::prelude::*;
use yew::prelude::*;

mod router;
mod routes;
mod components;
mod ui;
mod libs;
mod config;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}