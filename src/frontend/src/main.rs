use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <h1 class="text-3xl font-bold underline bg-red-200">{"HELLO WORLD"}</h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}