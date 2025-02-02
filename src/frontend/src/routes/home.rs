use yew::*;

use crate::components::side_bar::Sidebar;


#[function_component(Home)]
pub fn home() -> Html {
    
    let subicron_search_query = use_state(|| 0_i64);

    html! {
        <div class="flex min-h-full flex-row">
            <Sidebar/>
            <div class="
                transition-all
                transition-discrete
                ease-in-out
                duration-150

                flex min-h-full w-full
                flex-col justify-center items-center
                border-y-2 border-r-2 border-dashed
                border-r-purple-600 border-y-purple-600
                hover:border-r-purple-700 hover:border-y-purple-700
                rounded-xl">
                <h1>{ "Home" }</h1>
            </div>
        </div>
    }

}