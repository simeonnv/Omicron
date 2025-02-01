use web_sys::console;
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Protected)]
pub fn protected(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();

    // Use a state to track whether the token exists
    let has_token = use_state(|| {
        let token_exists = LocalStorage::get::<String>("token").is_ok();
        console::log_1(&format!("Token exists: {}", token_exists).into());
        token_exists
    });

    // Clone `has_token` before moving it into the closure
    let has_token_clone = has_token.clone();

    // Watch for changes to the token
    use_effect_with((), {
        move |_| {
            let has_token = has_token_clone.clone();
            let navigator = navigator.clone();

            // Check for the token whenever the component re-renders
            let token_exists = LocalStorage::get::<String>("token").is_ok();
            console::log_1(&format!("Token exists: {}", token_exists).into());
            has_token.set(token_exists);

            // Redirect if the token is missing
            if !token_exists {
                console::log_1(&"redirecting".to_string().into());
                navigator.push(&Route::Login);
            }

            // No cleanup needed
            || {}
        }
    });

    if *has_token {
        console::log_1(&"rendering children".to_string().into());
        html! { { for props.children.iter() } }
    } else {
        console::log_1(&"redirecting...".to_string().into());
        html! { <div>{"Redirecting..."}</div> }
    }
}