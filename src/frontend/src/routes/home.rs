use yew::*;

use crate::components::{mainbody::MainBody, sidebar::Sidebar};


#[function_component(Home)]
pub fn home() -> Html {
    
    let selected_subicron = use_state(|| 0_i64);

    html! {
        <div class="flex min-h-full flex-row">
            <Sidebar selected_subicron={selected_subicron.clone()}/>
            <MainBody selected_subicron={selected_subicron.clone()}/>
        </div>
    }

}