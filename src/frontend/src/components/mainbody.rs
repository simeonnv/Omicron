use web_sys::console;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{components::{post::Post, subicron::Subicron, welcome::Welcome}, libs::{request::{get_subicron_req::get_subicron_req, get_subicrons_req::get_subicrons_req}, structs::subicron::SubicronStruct}, ui::{button::Button, image::Image, input::Input}};

#[derive(Properties, PartialEq)]
pub struct MainBodyProps {
    pub selected_subicron: UseStateHandle<i64>,
    pub post_id: UseStateHandle<i64>, 
}


#[function_component(MainBody)]
pub fn home(props: &MainBodyProps) -> Html {

    html! {
        <div class="
            transition-all
            transition-discrete
            ease-in-out
            duration-150

            max-h-screen min-h-screen
            flex min-h-full w-full
            flex-col  items-center
            border-y-2 border-r-2 border-dashed
            border-r-purple-600 border-y-purple-600
            hover:border-r-purple-700 hover:border-y-purple-700
            rounded-xl">
            
            
            {
                if *props.selected_subicron == 0 {    
                    html!{ <Welcome/> }
                } else {
                    if *props.post_id == 0 {
                        html!{ <Subicron selected_subicron={props.selected_subicron.clone()} post_id={props.post_id.clone()}/> }
                    } else {
                        html!{ <Post subicron_id={props.selected_subicron.clone()} post_id={props.post_id.clone()}/> }
                    }
                }
            }
        
        </div>
    }
}
