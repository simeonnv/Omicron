use web_sys::console;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{libs::{request::get_subicrons_req::get_subicrons_req, structs::subicron::SubicronStruct}, ui::{button::Button, image::Image, input::Input}};

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub selected_subicron: UseStateHandle<i64>,
}


#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let subicron_search_query = use_state(|| String::new());
    let subicrons = use_state(|| Vec::<SubicronStruct>::new());

    // Callback for search query input change
    let on_search_change = {
        let subicron_search_query = subicron_search_query.clone();
        Callback::from(move |value: String| {
            subicron_search_query.set(value);
        })
    };

    {
        let subicron_search_query = subicron_search_query.clone();
        let subicrons = subicrons.clone();

        use_effect_with(subicron_search_query.to_string(), move |query| {
            let query = query.clone();
            let subicrons = subicrons.clone();

            spawn_local(async move {
                console::log_1(&format!("Fetching subicrons for query: {}", query).into());

                match get_subicrons_req(query).await {
                    Ok(fetched_subicrons) => {
                        console::log_1(&format!("Fetched {} subicrons", fetched_subicrons.len()).into());
                        subicrons.set(fetched_subicrons);
                    }
                    Err(e) => console::log_1(&format!("Failed to fetch subicrons: {}", e).into()),
                }
            });

            || ()
        });
    }

    html! {
        <div class="
            transition-all
            transition-discrete
            ease-in-out
            duration-150

            flex min-h-full flex-col 
            justify-start max-w-60 w-60 
            border-r-2 border-y-2
            rounded-r-xl
            border-dashed border-purple-600 hover:border-purple-800
            gap-4 p-4
            max-h-screen
        ">
            <h1 class="text-purple-600 pt-2 text-xl">{ "Subicrons" }</h1>
            
            <Input 
                value={(*subicron_search_query).clone()} 
                on_change={on_search_change} 
                placeholder="Search"
            />

            <div class="flex flex-col gap-4 scroll-smooth overflow-auto overflow-x-hidden pb-2 pt-2">
                {
                    subicrons.iter().map(|subicron| {
                        html!{
                            <div key={subicron.subicron_id.clone()} class="flex flex-row justify-start items-center gap-4">
                                <Image
                                    alt={subicron.name.clone()}
                                    image_id={subicron.image_id}
                                    class="h-6 w-6 min-h-6 min-w-6"
                                />
                                <p class="grow text-xs">{ &subicron.name }</p>
                                <Button 
                                    class={
                                        format!(
                                            "shrink h-min w-min !text-xs {}",
                                            if subicron.subicron_id == *props.selected_subicron { "!bg-purple-950" } else { "" }
                                        )
                                    }
                                    label=">"
                                    on_click={
                                        let selected_subicron = props.selected_subicron.clone();
                                        let subicron_id = subicron.subicron_id;
                                        Callback::from(move |_| {
                                            selected_subicron.set(subicron_id);
                                        })
                                    }
                                />
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
