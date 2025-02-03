use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct UpvoteButtonProps {

    pub is_upvoted: bool,

    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component(UpvoteButton)]
pub fn upvote_button(props: &UpvoteButtonProps) -> Html {
    let onclick = props.on_click.clone().unwrap_or_default();

    html! {
        <div {onclick} class="
            mx-4 border-2 border-purple-600 
            hover:border-purple-800 hover:-translate-y-0.5 
            rounded-lg transition-all
            transition-discrete ease-in-out
            duration-150 transform
        ">
            <svg class={format!("h-6 w-6 {} {}", if props.is_upvoted {"fill-purple-600"} else {""}, props.class.clone().unwrap_or("".to_string()))} xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="#e8eaed"><path d="M440-80v-647L256-544l-56-56 280-280 280 280-56 57-184-184v647h-80Z"/></svg>
        </div>
    }
}