use yew::{html, Html};
use yew_router::Routable;
use crate::{components::protected::Protected, routes::{login::Login, signup::Signup}};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/login")]
    Login,

    #[at("/signup")]
    Signup,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Protected>
                <h1>{ "Home" }</h1>
            </Protected>
        },
        
        Route::Login => html! { <Login /> },
        Route::Signup => html! { <Signup /> },

        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}