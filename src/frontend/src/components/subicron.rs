use web_sys::console;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{libs::{request::{get_posts_req::get_posts_req, get_subicron_req::get_subicron_req}, structs::{post::PostStruct, subicron::SubicronStruct}}, ui::{image::Image, input::Input, spinner::Spinner}};

#[derive(Properties, PartialEq)]
pub struct SubicronProps {
    pub selected_subicron: UseStateHandle<i64>,
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

    {
        let selected_subicron = props.selected_subicron.clone();
        let post_search_query = post_search_query.clone();
        let posts_ref = posts_ref.clone();

        use_effect_with(
            (*selected_subicron, (*post_search_query).clone()), move |(selected_subicron, post_search_query)| {

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
                            </div>

                            // pub struct PostStruct {
                            //     pub body: String,
                            //     pub created_at: String,
                            //     pub embed_id: Option<i64>,
                            //     pub header: String,
                            //     pub post_id: i64,
                            //     pub poster_id: i64,
                            //     pub subicron_id: i64,
                            //     pub poster_username: String,
                            //     pub upvotes: i64
                            // }

                            {
                                posts_ref.iter().map(|post| {
                                    html!{
                                        <div key={post.post_id.clone()} class="
                                            min-h-full w-full border border-purple-600 
                                            hover:border-purple-800 border-3 border-dashed
                                            transition-all transition-discrete
                                            ease-in-out duration-150 rounded-md p-4 flex flex-col">

                                            <div class="text-purple-600 text-xl w-full max-w-full overflow-hidden break-words">
                                                {post.header.clone()}
                                            </div>

                                            
                                            {
                                                if !post.embed_id.is_none(){
                                                    html! {
                                                        <div class="flex-1 w-full h-full rounded-md overflow-hidden">
                                                            <Image
                                                                alt={post.header.clone()}
                                                                image_id={post.embed_id}
                                                                class="w-full h-full object-cover !rounded-md border-2"
                                                            />
                                                        </div>
                                                    }
                                                } else {
                                                    html! {
                                                        <div class="flex-1 w-full h-full rounded-md overflow-visible whitespace-normal break-words">
                                                            {post.body.clone()}
                                                        </div>
                                                    }
                                                }
                                                
                                            }
                                            
                                            
                                            <div class="flex flex-row items-center py-4">
                                                <p>
                                                    {format!("By {}", post.poster_username)}
                                                </p>
                                                <div class="grow"/>
                                                
                                                <div class="
                                                    mx-4 border-2 border-purple-600 
                                                    hover:border-purple-800 hover:-translate-y-0.5 
                                                    rounded-lg transition-all
                                                    transition-discrete ease-in-out
                                                    duration-150 transform
                                                ">
                                                    <svg class={format!("h-6 w-6 {}", if post.is_upvoted {"fill-purple-600"} else {""})} xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="#e8eaed"><path d="M440-80v-647L256-544l-56-56 280-280 280 280-56 57-184-184v647h-80Z"/></svg>
                                                </div>

                                                <p class="text-purple-600">
                                                    {format!("Upvotes {}", post.upvotes)}
                                                </p>
                                            </div>

                                        </div>
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
