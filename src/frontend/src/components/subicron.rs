use web_sys::console;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{components::post_preview::PostPreview, libs::{request::{get_posts_req::get_posts_req, get_subicron_req::get_subicron_req, post_post_req::post_post_req}, structs::{post::PostStruct, subicron::SubicronStruct}}, ui::{input::Input, modal::{FormSubmission, FormType, Modal}, spinner::Spinner}};

#[derive(Properties, PartialEq)]
pub struct SubicronProps {
    pub selected_subicron: UseStateHandle<i64>,
    pub post_id: UseStateHandle<i64>
}


#[function_component(Subicron)]
pub fn subicron(props: &SubicronProps) -> Html {
    
    let subicron_ref = use_state(|| None::<SubicronStruct>);
    let posts_ref = use_state(|| Vec::<PostStruct>::new());
    let post_search_query = use_state(|| String::new());
    
    {
        let selected_subicron = props.selected_subicron.clone();
        let subicron_ref = subicron_ref.clone();

        use_effect_with(selected_subicron, move |selected_subicron| {
            let selected_subicron = selected_subicron.clone();
            let subicron_ref = subicron_ref.clone();

            subicron_ref.set(None);

            spawn_local(async move {
                console::log_1(&format!("selected subicron is: {}", *selected_subicron).into());
                
                match get_subicron_req((*selected_subicron).to_string()).await {
                    Ok(fetched_subicron) => {
                        // console::log_1(&format!("Fetched subicron: {}", serde_json::to_string(&fetched_subicron).unwrap_or("".to_string())).into());
                        subicron_ref.set(Some(fetched_subicron));
                    }
                    Err(e) => console::log_1(&format!("Failed to fetch subicrons: {}", e).into()),
                }
            });

            || ()
        });
    }

    let posts_hook = use_state(|| 0_i64);

    {
        let selected_subicron = props.selected_subicron.clone();
        let post_search_query = post_search_query.clone();
        let posts_ref = posts_ref.clone();
        let posts_hook = posts_hook.clone();

        use_effect_with(
            (*selected_subicron, (*post_search_query).clone(), *posts_hook), move |(selected_subicron, post_search_query, _)| {

                let selected_subicron = selected_subicron.clone();
                let post_search_query = post_search_query.clone();
                let posts_ref = posts_ref.clone();

                spawn_local(async move {
                    console::log_1(&format!("Fetching post for query: {}", post_search_query).into());
                    
                    match get_posts_req(post_search_query, selected_subicron).await {
                        Ok(fetched_subicrons) => {
                            console::log_1(&format!("Fetched {} posts", fetched_subicrons.len()).into());
                            posts_ref.set(fetched_subicrons);
                        }
                        Err(e) => console::log_1(&format!("Failed to fetch subicrons: {}", e).into()),
                    }
                });
                || () // No cleanup function
            },
        );
    }

    let handle_submit = {
        let selected_subicron = *props.selected_subicron; // Clone the value to avoid moving the reference
        let posts_hook = posts_hook.clone();
    
        Callback::from(move |submission: FormSubmission| {
            match submission {
                FormSubmission::Comment { content, image } => {
                    console::log_1(&format!("You are stupid: {}", content).into());
                }
                FormSubmission::Post { title, content, image } => {
                    console::log_1(&format!("Creating post: {}", content).into());
    
                    let posts_hook = posts_hook.clone(); // Clone inside async block to avoid move issues
                    let selected_subicron = selected_subicron; // Use the cloned value
    
                    spawn_local(async move {
                        match post_post_req(selected_subicron, title, content, image).await {
                            Ok(_) => {
                                posts_hook.set(*posts_hook + 1); // Now we use the cloned state
                            }
                            Err(e) => console::log_1(&format!("Failed to post: {}", e).into()),
                        }
                    });
                }
            }
        })
    };

    html! {
        <div class="max-h-screen min-h-screen h-full w-full">
            {
                if subicron_ref.is_none() {
                    html!{
                        <div class="align-middle justify-center items-center h-full flex flex-col gap-4">
                            <Spinner class="h-16 w-16"/>
                        </div>
                    }
                } else {
                    let subicron = subicron_ref.as_ref().cloned().unwrap_or(SubicronStruct {
                        created_at: "null".to_string(),
                        image_id: None,
                        name: "null".to_string(),
                        subicron_id: 0,
                    });

                    
                    let on_search_change = {
                        let subicron_search_query = post_search_query.clone();
                        Callback::from(move |value: String| {
                            subicron_search_query.set(value);
                        })
                    };

                    html!{
                        <div class="
                            flex flex-col py-2
                            grid grid-cols-2 gap-4
                            w-full scroll-smooth 
                            overflow-auto overflow-x-hidden 
                            max-h-full p-16 
                        ">
                            
                            <div class="
                                col-span-2
                                py-8 border-b-2 border-purple-600 
                                hover:border-purple-800
                                border-dashed rounded-xl
                                flex flex-col gap-4">
                                <h1 class="text-purple-600 text-2xl" >{ format!("Welcome to {}", subicron.name) }</h1>
                                <h1 class="text-purple-600 text-xl" >{ "Browse any post you want!" }</h1>
                                
                                <div>
                                    <Input 
                                        value={(*post_search_query).clone()} 
                                        on_change={on_search_change} 
                                        placeholder="Search"
                                        class="!w-min !p-1 "
                                    />
                                </div>
                                <div class="flex flex-row lign-middle w-full justify-center items-center">
                                    <Modal 
                                        form_type={FormType::Post} 
                                        on_submit={handle_submit.clone()} 
                                    />
                                </div>
                            </div>

                            

                            {
                                posts_ref.iter().map(|post| {
                                    html!{
                                        <PostPreview post={post.clone()} post_id={props.post_id.clone()}/>
                                    }
                                }).collect::<Html>()
                            }
                        
                        </div>
                    }
                }

            }
        </div>
    }
}
