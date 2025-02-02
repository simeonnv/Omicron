use yew::prelude::*;
use web_sys::{window, HtmlElement}; // Import web_sys for DOM manipulation

use crate::ui::spinner::Spinner;

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    pub alt: String,
    pub image_id: Option<i64>,

    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component(Image)]
pub fn image(props: &ButtonProps) -> Html {
    let img_ref = use_node_ref(); // Reference to the <img> element
    let classes = format!("rounded-full border border-purple-600 {}", props.class.clone().unwrap_or_default());

    // me when error
    let onerror = {
        let img_ref = img_ref.clone();
        let classes = classes.clone();
        Callback::from(move |_| {
            if let Some(img) = img_ref.cast::<HtmlElement>() {
                let document = window().unwrap().document().unwrap();
                
                let spinner = document
                    .create_element("div")
                    .unwrap();
                spinner.set_attribute("class", &classes).unwrap();
                
                // Render the Spinner
                yew::Renderer::<Spinner>::with_root_and_props(spinner.clone(), Default::default()).render();

                // Replace the failed image (real)
                img.replace_with_with_node_1(&spinner).unwrap();
            }
        })
    };

    html! {
        <img
            ref={img_ref}
            // src={props.image_id.clone()}
            alt={props.alt.clone()}
            class={classes}
            onerror={onerror}
        />
    }
}
