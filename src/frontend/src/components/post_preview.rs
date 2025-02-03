use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::{
    libs::{
        request::{get_posts_req::get_posts_req, get_subicron_req::get_subicron_req, get_upvotes_req::get_upvotes_req},
        structs::{post::PostStruct, subicron::SubicronStruct, upvotes_struct::UpvotesStruct},
    },
    ui::{image::Image, input::Input, spinner::Spinner},
};

#[derive(Properties, PartialEq)]
pub struct PostPreviewProps {
    pub post: PostStruct,
}

#[function_component(PostPreview)]
pub fn post_preview(props: &PostPreviewProps) -> Html {

    let upvotes_ref = use_state(|| None::<UpvotesStruct>);
    let upvote_button_ref = use_state(|| 0_i64);

    {
        let subicron_id = props.post.subicron_id.clone();
        let post_id = props.post.post_id.clone();
        let upvotes_ref = upvotes_ref.clone();
        let upvote_button_ref = upvote_button_ref.clone();

        use_effect_with(upvote_button_ref, move |selected_subicron| {
            let subicron_id = subicron_id.clone();
            let post_id = post_id.clone();
            let upvotes_ref = upvotes_ref.clone();

            upvotes_ref.set(None);

            spawn_local(async move {

                match get_upvotes_req(subicron_id, post_id).await {
                    Ok(fetched_upvotes) => {
                        // console::log_1(&format!("Fetched subicron: {}", serde_json::to_string(&fetched_subicron).unwrap_or("".to_string())).into());
                        upvotes_ref.set(Some(fetched_upvotes));
                    }
                    Err(e) => console::log_1(&format!("Failed to fetch subicrons: {}", e).into()),
                }
            });

            || ()
        });
    }


    html! {
        <div key={props.post.post_id.clone()} class="
            min-h-full w-full border border-purple-600 
            hover:border-purple-800 border-3 border-dashed
            transition-all transition-discrete
            ease-in-out duration-150 rounded-md p-4 flex flex-col">

            <div class="text-purple-600 text-xl w-full max-w-full overflow-hidden break-words">
                {props.post.header.clone()}
            </div>
                
            {
                if !props.post.embed_id.is_none(){
                    html! {
                        <div class="flex-1 w-full h-full rounded-md overflow-hidden">
                            <Image
                                alt={props.post.header.clone()}
                                image_id={props.post.embed_id}
                                class="w-full h-full object-cover !rounded-md border-2"
                            />
                        </div>
                    }
                } else {
                    html! {
                        <div class="flex-1 w-full h-full rounded-md overflow-visible whitespace-normal break-words">
                            {props.post.body.clone()}
                        </div>
                    }
                }
            }

            <div class="flex flex-row items-center py-4">
                <p>
                    {format!("By {}", props.post.poster_username)}
                </p>
                
                <div class="grow"/>

                <div class="
                    mx-4 border-2 border-purple-600 
                    hover:border-purple-800 hover:-translate-y-0.5 
                    rounded-lg transition-all
                    transition-discrete ease-in-out
                    duration-150 transform
                ">
                    <svg class={format!("h-6 w-6 {}", if true {"fill-purple-600"} else {""})} xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="#e8eaed"><path d="M440-80v-647L256-544l-56-56 280-280 280 280-56 57-184-184v647h-80Z"/></svg>
                </div>
                
                <p class="text-purple-600">
                    {format!("Upvotes {}", 15)}
                </p>
            </div>

        </div>
    }
}
