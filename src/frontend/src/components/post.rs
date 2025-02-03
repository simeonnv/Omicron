use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::{
    libs::{
        request::{downvote_req::downvote_req, get_post_req::get_post_req, get_upvotes_req::get_upvotes_req, upvote_req::upvote_req},
        structs::{post::PostStruct, upvotes_struct::UpvotesStruct},
    },
    ui::{image::Image, spinner::Spinner},
};

#[derive(Properties, PartialEq)]
pub struct PostPreviewProps {
    pub subicron_id: UseStateHandle<i64>,
    pub post_id: UseStateHandle<i64>, 
}

#[function_component(Post)]
pub fn post(props: &PostPreviewProps) -> Html {

    let post_ref = use_state(|| None::<PostStruct>);

    {
        let subicron_id = props.subicron_id.clone();
        let post_id = props.post_id.clone();
        let post_ref = post_ref.clone();

        use_effect_with(post_id.clone(), move |_| {
            let subicron_id = subicron_id.clone();
            let post_id = post_id.clone();
            let post_ref = post_ref.clone();

            spawn_local(async move {

                match get_post_req(*subicron_id, *post_id).await {
                    Ok(fetched_post) => {
                        // console::log_1(&format!("Fetched subicron: {}", serde_json::to_string(&fetched_subicron).unwrap_or("".to_string())).into());
                        post_ref.set(Some(fetched_post));
                    }
                    Err(e) => console::log_1(&format!("Failed to fetch subicrons: {}", e).into()),
                }
            });

            || ()
        });
    }

    if !(*post_ref).is_none() {
        let post_ref = (*post_ref).clone().unwrap_or_default();

        html! {
            <div>
                <h1>
                    {post_ref.body}
                </h1>
            </div>
        }
    } else {
        html! {
            <div>
                <Spinner/>
            </div>
        }
    }
}