use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct InputProps {
    pub value: String,
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let oninput = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            on_change.emit(input.value());
        })
    };

    html! {
        <input
            type="text"
            class={format!("

                transition-all
                transition-discrete
                ease-in-out
                duration-150

                transform
                hover:-translate-y-1

                text-middle placeholder:text-center 
                border border-purple-600 rounded-lg 
                p-2 focus:outline-2 focus:outline-offset-2
                focus:outline-violet-700
             {}", props.class.clone().unwrap_or_default())}
            value={props.value.clone()}
            {oninput}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
        />
    }
}