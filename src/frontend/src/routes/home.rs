use yew::*;

use crate::components::{mainbody::MainBody, sidebar::Sidebar};


#[function_component(Home)]
pub fn home() -> Html {
    let selected_subicron = use_state(|| 1_i64);
    let post_id = use_state(|| 1_i64);

    html! {
        <div class="flex h-screen flex-row">
            <Sidebar selected_subicron={selected_subicron.clone()} post_id={post_id.clone()}/>
            <MainBody selected_subicron={selected_subicron.clone()} post_id={post_id.clone()}/>
        </div>
    }
}