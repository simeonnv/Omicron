use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

use crate::{
    libs::{
        request::{downvote_req::downvote_req, get_post_req::get_post_req, get_upvotes_req::get_upvotes_req, upvote_req::upvote_req},
        structs::{comment::CommentStruct, post::PostStruct, upvotes_struct::UpvotesStruct},
    },
    ui::{image::Image, post_button::PostButton, spinner::Spinner, upvote_button::UpvoteButton},
};

#[derive(Properties, PartialEq)]
pub struct PostPreviewProps {
    pub comment: CommentStruct
}

#[function_component(Comment)]
pub fn comment(props: &PostPreviewProps) -> Html {

    html! {
        <div class="flex flex-row justify-middle items-center content-center">
            <div class="grow"/>
            <div class="w-min flex gap-4 flex-col border-2 rounded-xl border-dashed border-purple-600 hover:border-purple-800 p-8 justify-middle items-center content-center">
                <p class="text-purple-600 text-2xl">
                    {props.comment.text.clone()}
                </p>
                <p class="text-purple-600 text-2xl">
                    if !props.comment.embed_id.is_none() {
                        <Image class="!rounded-xl" alt={props.comment.text.clone()} image_id={props.comment.embed_id.unwrap_or_default()} />
                    }
                </p>
                <div class="text-purple-600 text-lg flex flex-row">
                    <p>{props.comment.created_at.clone()}</p>
                    <div class="grow min-w-32"/>
                    <p>{format!("By {}", props.comment.commenter_username.clone())}</p>
                </div>
            </div>
            <div class="grow"/>
        </div>
    }

}