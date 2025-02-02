use yew::prelude::*;


#[function_component(Welcome)]
pub fn welcome() -> Html {

    html! {
        <div class="align-middle justify-center items-center h-full flex flex-col gap-4">
            <h1 class="text-purple-600 text-2xl">{ "Welcome to Omicron" }</h1>
            <h2 class="text-purple-600 text-2xl">{ "Feel free to browse any subicron you wish!" }</h2>
            <img class="h-80 w-80 border-4 !border-purple-600 border-dashed rounded-xl" src="/images/tigurche.gif"/>
        </div>
    }
}
