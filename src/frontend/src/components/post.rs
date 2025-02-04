use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::{
    components::comment::Comment, libs::{
        request::{downvote_req::downvote_req, get_comments_req::get_comments_req, get_post_req::get_post_req, get_upvotes_req::get_upvotes_req, post_comment_req::post_comment_req, upvote_req::upvote_req},
        structs::{comment::CommentStruct, post::PostStruct, upvotes_struct::UpvotesStruct},
    }, ui::{image::Image, modal::{FormSubmission, FormType, Modal}, post_button::PostButton, spinner::Spinner, upvote_button::UpvoteButton}
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

    let upvotes_ref = use_state(|| None::<UpvotesStruct>);
    let upvote_button_ref = use_state(|| 0_i64);

    {
        let subicron_id = props.subicron_id.clone();
        let post_id = props.post_id.clone();
        let upvotes_ref = upvotes_ref.clone();
        let upvote_button_ref = upvote_button_ref.clone();

        use_effect_with(upvote_button_ref, move |_| {
            let subicron_id = subicron_id.clone();
            let post_id = post_id.clone();
            let upvotes_ref = upvotes_ref.clone();

            upvotes_ref.set(None);

            spawn_local(async move {

                match get_upvotes_req(*subicron_id, *post_id).await {
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
        let subicron_id = props.subicron_id.clone();
        let post_id = props.post_id.clone();
        let upvote_button_ref = upvote_button_ref.clone();

        Callback::from(move |_| {
            let upvotes = (*upvotes_ref).clone().unwrap_or_default();

            let subicron_id = subicron_id.clone();
            let post_id = post_id.clone();
            let upvote_button_ref = upvote_button_ref.clone();
            let upvotes_ref = upvotes_ref.clone();

            spawn_local(async move {
                if upvotes.is_upvoted {
                    match downvote_req(*subicron_id, *post_id).await {
                        Ok(fetched_upvotes) => {
                            console::log_1(&format!("upvoted: : {}", fetched_upvotes).into());
                            upvote_button_ref.set(*upvote_button_ref + 1);
                        }
                        Err(e) => console::log_1(&format!("Failed to fetch subicrons: {}", e).into()),
                    }
                } else {
                    match upvote_req(*subicron_id, *post_id).await {
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

    let on_exit = {
        let post_id = props.post_id.clone();
        
        Callback::from(move |_| {
            post_id.set(0)
        })
    };

    let comments_ref = use_state(|| Vec::<CommentStruct>::new());
    let comment_hook = use_state(|| 0_i64);

    {
        let subicron_id = props.subicron_id.clone();
        let post_id = props.post_id.clone();
        let comments_ref = comments_ref.clone();
        let comment_hook = comment_hook.clone();

        use_effect_with((post_id.clone(), comment_hook.clone()), move |_| {
            let subicron_id = subicron_id.clone();
            let post_id = post_id.clone();
            let comments_ref = comments_ref.clone();

            spawn_local(async move {

                match get_comments_req(*subicron_id, *post_id).await {
                    Ok(fetched_post) => {
                        // console::log_1(&format!("Fetched subicron: {}", serde_json::to_string(&fetched_subicron).unwrap_or("".to_string())).into());
                        comments_ref.set(fetched_post);
                    }
                    Err(e) => console::log_1(&format!("Failed to fetch subicrons: {}", e).into()),
                }
            });

            || ()
        });
    }

    let handle_submit = {
        let comment_hook = comment_hook.clone();
        let subicron_id = *props.subicron_id; // Clone before async block
        let post_id = *props.post_id; // Clone before async block
    
        Callback::from(move |submission: FormSubmission| {
            match submission {
                FormSubmission::Post { title, content, image } => {
                    console::log_1(&format!("You are stupid: {} - {}", title, content).into());
                }
                FormSubmission::Comment { content, image } => {
                    console::log_1(&format!("Creating comment: {}", content).into());
    
                    let comment_hook = comment_hook.clone(); // Clone inside async block to avoid move issues
    
                    spawn_local(async move {
                        match post_comment_req(subicron_id, post_id, content, None).await {
                            Ok(_) => {
                                comment_hook.set(*comment_hook + 1); // Now we use the cloned state
                            }
                            Err(e) => console::log_1(&format!("Failed to post comment: {}", e).into()),
                        }
                    });
                }
            }
        })
    };
    
    

    

    if !(*post_ref).is_none() {
        let post_ref = (*post_ref).clone().unwrap_or_default();

        html! {
            <div class="flex flex-col justify-start w-full h-full p-8 gap-4 scroll-smooth overflow-auto overflow-x-hidden">
                <div class="w-full flex flex-row align-bottom justify-items-start justify-start">
                    <PostButton on_click={on_exit} class="!h-6 !w-6 rotate-270"/>
                    <div class="grow"/>
                </div>
                <div class="w-full flex flex-row justify-center items-center text-2xl text-purple-600">
                    {post_ref.header.clone()}
                </div>
                <div class="flex flex-row justify-center items-center text-2xl text-purple-600 break-all">
                    {post_ref.body.clone()}
                </div>
                {
                    

                    if post_ref.embed_id.is_none() {
                        let image_id = post_ref.embed_id.unwrap_or(1);
                        let upvotes = (*upvotes_ref).clone().unwrap_or_default();

                        html! {
                            <div class="w-full flex flex-col justify-center items-center text-2xl text-purple-600">
                                <Image
                                    alt={post_ref.header.clone()}
                                    image_id={image_id}
                                    class="h-96 w-96 min-h-96 min-w-96 !rounded-md"
                                />
                                <div class="flex flex-row w-full">
                                    <div class="flex flex-col justify-start justify-items-start content-start items-start">
                                        <p>{format!("By {}", post_ref.poster_username)}</p>
                                        <p>{format!("Posted on {}", post_ref.created_at)}</p>
                                    </div>
                                    
                                    <div class="grow"/>
                                    <div class="flex flex-row justify-center justify-items-center content-center items-center">
                                        <UpvoteButton is_upvoted={upvotes.is_upvoted} on_click={on_upvote}/>
                                        
                                        <p class="text-purple-600">
                                            {format!("Upvotes {}", upvotes.upvotes)} 
                                        </p>
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        let upvotes = (*upvotes_ref).clone().unwrap_or_default();

                        html! {
                            <div class="w-full flex flex-col justify-center items-center text-2xl text-purple-600">
                                <UpvoteButton is_upvoted={upvotes.is_upvoted} on_click={on_upvote}/>
                                    
                                <p class="text-purple-600">
                                    {format!("Upvotes {}", upvotes.upvotes)} 
                                </p>
                            </div>
                        }
                    }
                }
                <div class="flex flex-row">
                    <h1 class="text-purple-600 text-xl">{"Comments:"}</h1>
                    <div class="grow"/>
                    <Modal 
                        form_type={FormType::Comment} 
                        on_submit={handle_submit.clone()} 
                    />
                </div>
                {
                    comments_ref.iter().map(|comment| {
                        html!{
                            <Comment comment={comment.clone()}/>
                        }
                    }).collect::<Html>()
                }

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