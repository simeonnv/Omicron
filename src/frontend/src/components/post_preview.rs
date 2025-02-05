use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::{
    libs::{
        request::{downvote_req::downvote_req, get_upvotes_req::get_upvotes_req, upvote_req::upvote_req},
        structs::{post::PostStruct, upvotes_struct::UpvotesStruct},
    },
    ui::{post_button::PostButton, image::Image, spinner::Spinner, upvote_button::UpvoteButton},
};

#[derive(Properties, PartialEq)]
pub struct PostPreviewProps {
    pub post: PostStruct,
    pub post_id: UseStateHandle<i64>, 
}

#[function_component(PostPreview)]
pub fn post_preview(props: &PostPreviewProps) -> Html {

    let upvotes_ref = use_state(|| None::<UpvotesStruct>);
    let upvote_button_ref = use_state(|| 0_i64);
    let subicron_id = props.post.subicron_id.clone();
    let post_id = props.post.post_id.clone();

    {
        let subicron_id = props.post.subicron_id.clone();
        let post_id = props.post.post_id.clone();
        let upvotes_ref = upvotes_ref.clone();
        let upvote_button_ref = upvote_button_ref.clone();

        use_effect_with(upvote_button_ref, move |_| {
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
    
    let on_upvote = {
        let upvotes_ref = upvotes_ref.clone();
        let subicron_id = subicron_id.clone();
        let post_id = post_id.clone();
        let upvote_button_ref = upvote_button_ref.clone();

        Callback::from(move |_| {
            let upvotes = (*upvotes_ref).clone().unwrap_or_default();

            let subicron_id = subicron_id.clone();
            let post_id = post_id.clone();
            let upvote_button_ref = upvote_button_ref.clone();
            let upvotes_ref = upvotes_ref.clone();

            spawn_local(async move {
                if upvotes.is_upvoted {
                    match downvote_req(subicron_id, post_id).await {
                        Ok(fetched_upvotes) => {
                            console::log_1(&format!("upvoted: : {}", fetched_upvotes).into());
                            upvote_button_ref.set(*upvote_button_ref + 1);
                        }
                        Err(e) => console::log_1(&format!("Failed to fetch subicrons: {}", e).into()),
                    }
                } else {
                    match upvote_req(subicron_id, post_id).await {
                        Ok(fetched_upvotes) => {
                            console::log_1(&format!("upvoted: : {}", fetched_upvotes).into());
                            upvote_button_ref.set(*upvote_button_ref + 1);
                        }
                        Err(e) => console::log_1(&format!("Failed to fetch subicrons: {}", e).into()),
                    }
                }
                // Set the new state after upvote/downvote
                upvotes_ref.set(Some(UpvotesStruct {
                    is_upvoted: !upvotes.is_upvoted,
                    upvotes: if upvotes.is_upvoted { upvotes.upvotes - 1 } else { upvotes.upvotes + 1 },
                }));
            });
        })
    };

    let on_enter = {
        let post = props.post.clone();
        let post_id = props.post_id.clone();
        

        Callback::from(move |_| {
            post_id.set(post.post_id)
        })
    };


    html! {
        <div key={props.post.post_id.clone()} class="
            min-h-full w-full border border-purple-600 
            hover:border-purple-800 border-3 border-dashed
            transition-all transition-discrete
            ease-in-out duration-150 rounded-md p-4 flex flex-col">

            <div class="relative flex flex-row items-center justify-center w-full">
                <div class="text-purple-600 text-xl text-center">
                    {props.post.header.clone()}
                </div>

                <div class="absolute right-0">
                    <PostButton on_click={on_enter}/>
                </div>
            </div>
                
            {
                if !props.post.embed_id.is_none(){
                    html! {
                        <div class="flex-1 w-full h-full rounded-md overflow-hidden">
                            <Image
                                alt={props.post.header.clone()}
                                image_id={props.post.embed_id.unwrap_or_default()}
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

            {if !(*upvotes_ref).is_none() {

                    let upvotes = (*upvotes_ref).clone().unwrap_or_default();

                    html! {
                        <div class="flex flex-row min-h-16 items-center py-4">
                            <p>
                                {format!("By {}", props.post.poster_username)}
                            </p>
                            
                            <div class="grow"/>
                            
                            <UpvoteButton is_upvoted={upvotes.is_upvoted} on_click={on_upvote}/>
                            
                            <p class="text-purple-600">
                                {format!("Upvotes {}", upvotes.upvotes)} 
                            </p>
                        </div>
                    }
                } else {
                    html!{
                        <div class="flex flex-row min-h-16 items-center py-4"> 
                            <div class="grow"/>
                            <Spinner class="h-8 w-8"/>
                            <div class="grow"/> 
                        </div>
                    }
                }
            }
        </div>
    }
}
