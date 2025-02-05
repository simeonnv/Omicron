use wasm_bindgen_futures::spawn_local;
use web_sys::{console, js_sys::{Array, Uint8Array}, Url};
use yew::prelude::*;
use crate::{libs::request::get_file_req::get_file_req, ui::spinner::Spinner};

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    pub alt: String,
    pub image_id: i64,
    #[prop_or_default]
    pub class: Option<String>,
}

#[function_component(Image)]
pub fn image(props: &ButtonProps) -> Html {
    let url = use_state(|| None::<String>);
    let error = use_state(|| false);
    let classes = format!(
        "rounded-full border border-purple-600 {}",
        props.class.clone().unwrap_or_default()
    );

    // Reset error state when image_id changes
    {
        let error = error.clone();
        use_effect_with(props.image_id, move |_| {
            error.set(false);
            || ()
        });
    }

    {
        let url = url.clone();
        let error = error.clone();
        let image_id = props.image_id; // Clone the image_id to avoid borrowing issues
        
        use_effect_with(image_id, move |&image_id| {
            let url = url.clone();
            let error = error.clone();

            spawn_local(async move {
                match get_file_req(image_id).await {
                    Ok(fetched_image) => {
                        let data = fetched_image.file_blob;
                        let array = Uint8Array::from(data.as_slice());
                        let blob = web_sys::Blob::new_with_u8_array_sequence(&Array::of1(&array)).unwrap();
                        let object_url = Url::create_object_url_with_blob(&blob).unwrap();
                        url.set(Some(object_url));
                        error.set(false);
                    }
                    Err(e) => {
                        console::log_1(&format!("Failed to fetch file: {}", e).into());
                        error.set(true);
                    }
                }
            });

            || ()
        });
    }

    let onerror = {
        let error = error.clone();
        Callback::from(move |_| {
            error.set(true);
        })
    };

    html! {
        if url.is_none() || *error {
            <div class={classes}>
                <Spinner />
            </div>
        } else {
            <img
                src={url.as_ref().unwrap().clone()}
                alt={props.alt.clone()}
                class={classes}
                onerror={onerror}
            />
        }
    }
}