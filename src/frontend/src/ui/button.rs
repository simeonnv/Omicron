use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    pub label: String,

    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let onclick = props.on_click.clone().unwrap_or_default();

    html! {
        <button 
            class="
                transition-all
                transition-discrete
                ease-in-out
                duration-150

                transform
                hover:-translate-y-1

                flex w-full justify-center 
                rounded-md bg-purple-600 px-3 
                py-1.5 text-sm/6 font-semibold 
                text-white shadow-xs 
                hover:bg-purple-700 
                focus-visible:outline-2 
                focus-visible:outline-offset-2 
                focus-visible:outline-black-600
                focus:outline-2 focus:outline-offset-2 
                focus:outline-violet-500 active:bg-violet-800
                
            " 
            {onclick}
        >
            {&props.label}
        </button>
    }
}