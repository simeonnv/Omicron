use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{js_sys::{Array, Uint8Array}, FileReader, HtmlInputElement, HtmlTextAreaElement, Url};
use yew::prelude::*;

use crate::libs::request::post_file_req::post_file_req;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub form_type: FormType,
    pub on_submit: Callback<FormSubmission>,
}


#[derive(Clone, PartialEq)]
pub enum FormType {
    Post,
    Comment,
}

#[derive(Clone, PartialEq)]
pub enum FormSubmission {
    Post { 
        title: String, 
        content: String,
        image: Option<i64>,
    },
    Comment { 
        content: String,
        image: Option<i64>, // Add image field to comments
    },
}

// Add this new component for rendering blob images
#[derive(Properties, PartialEq)]
pub struct BlobImageProps {
    pub data: Vec<u8>,
}

#[function_component(BlobImage)]
pub fn blob_image(props: &BlobImageProps) -> Html {
    let url = use_state(|| None::<String>);
    
    // Fixed use_effect_with usage
    use_effect_with(props.data.clone(), {
        let data = props.data.clone();
        let url = url.clone();
        move |_| {
            let array = Uint8Array::from(data.as_slice());
            let blob = web_sys::Blob::new_with_u8_array_sequence(&Array::of1(&array)).unwrap();
            let object_url = Url::create_object_url_with_blob(&blob).unwrap();
            url.set(Some(object_url.clone()));
            
            // Return cleanup closure
            move || {
                Url::revoke_object_url(&object_url).unwrap();
            }
        }
    });

    html! {
        if let Some(url) = &*url {
            <img src={url.clone()} class="max-w-full h-auto rounded-lg mt-2" />
        }
    }
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let is_open = use_state(|| false);
    let title = use_state(|| String::new());
    let content = use_state(|| String::new());
    let image_data = use_state(|| None::<Vec<u8>>);
    let preview_url = use_state(|| None::<String>);

    let form_type = props.form_type.clone();
    let on_submit = props.on_submit.clone();

    // Image upload handler
    let on_image_change = {
        // Clone the state handles outside the main closure
        let image_data = image_data.clone();
        let preview_url = preview_url.clone();
    
        Callback::from(move |e: Event| {
            // Clone the state handles again inside the event handler
            let image_data = image_data.clone();
            let preview_url = preview_url.clone();
    
            let input = e.target_unchecked_into::<HtmlInputElement>();
            if let Some(files) = input.files() {
                if files.length() > 0 {
                    if let Some(file) = files.get(0) {
                        let reader = FileReader::new().unwrap();
                        let reader_clone = reader.clone();
    
                        // Clone the state handles for the onload closure
                        let image_data = image_data.clone();
                        let preview_url = preview_url.clone();
    
                        let onload = Closure::once(move |_e: web_sys::ProgressEvent| {
                            let result = reader_clone.result().unwrap();
                            let array = Uint8Array::new(&result);
                            let bytes = array.to_vec();
    
                            // Update state using the cloned handles
                            image_data.set(Some(bytes.clone()));
    
                            // Create blob URL
                            let blob = web_sys::Blob::new_with_u8_array_sequence(&Array::of1(&array))
                                .expect("Failed to create blob");
                            let url = Url::create_object_url_with_blob(&blob)
                                .expect("Failed to create object URL");
                            preview_url.set(Some(url));
                        });
    
                        reader.read_as_array_buffer(&file).unwrap();
                        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                        onload.forget();
                    }
                }
            }
        })
    };



    let onsubmit = {
        let is_open = is_open.clone();
        let title = title.clone();
        let content = content.clone();
        let image_data = image_data.clone();
        let preview_url = preview_url.clone();
        let form_type = form_type.clone();
        let on_submit = on_submit.clone();
    
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
    
            // Clone the state variables inside the closure to avoid moving the original captures
            let is_open = is_open.clone();
            let title = title.clone();
            let content = content.clone();
            let image_data = image_data.clone();
            let preview_url = preview_url.clone();
            let form_type = form_type.clone();
            let on_submit = on_submit.clone();
    
            spawn_local(async move {
                let mut image_id = None;
    
                if let Some(image) = image_data.as_ref() {
                    match post_file_req(image.clone()).await {
                        Ok(id) => image_id = Some(id),
                        Err(err) => {
                            web_sys::console::log_1(&format!("Error uploading image: {}", err).into());
                        }
                    }
                }
    
                let submission = match form_type {
                    FormType::Post => FormSubmission::Post {
                        title: (*title).clone(),
                        content: (*content).clone(),
                        image: image_id,
                    },
                    FormType::Comment => FormSubmission::Comment {
                        content: (*content).clone(),
                        image: image_id,
                    },
                };
    
                on_submit.emit(submission);
    
                // Update state with cloned handles
                is_open.set(false);
                title.set(String::new());
                content.set(String::new());
                image_data.set(None);
                preview_url.set(None);
            });
        })
    };


    let title_handler = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    // Modal open/close handlers
    let open_modal = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(true))
    };

    let close_modal = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(false))
    };

    html! {
        <>
            <button 
                onclick={open_modal} 
                class="transition-all transition-discrete ease-in-out duration-150
                       transform hover:-translate-y-0.5 flex w-full justify-center 
                       rounded-md bg-purple-600 px-3 py-1.5 text-sm/6 font-semibold 
                       text-white shadow-xs hover:bg-purple-700 focus:outline-2 
                       focus:outline-offset-2 focus:outline-violet-500 active:bg-violet-800 w-min"
            >
                {match props.form_type {
                    FormType::Post => "Post!",
                    FormType::Comment => "Comment!",
                }}
            </button>

            if *is_open {
                <div class="fixed inset-0 bg-stone-950/70 flex items-center justify-center p-4 z-50">
                    <div class="bg-stone-950 rounded-xl shadow-lg w-full max-w-lg">
                        <form {onsubmit} class="p-6">
                            <div class="flex justify-between items-center mb-4">
                                <h2 class="text-xl text-purple-600 font-semibold">
                                    {match props.form_type {
                                        FormType::Post => "Create Post",
                                        FormType::Comment => "Create Comment",
                                    }}
                                </h2>
                                <button 
                                    type="button" 
                                    onclick={close_modal.clone()}
                                    class="text-purple-500 hover:text-purple-700"
                                >
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                                    </svg>
                                </button>
                            </div>

                            {match props.form_type {
                                FormType::Post => html! {
                                    <>
                                        <div class="mb-4 gap-2">
                                            <label class="block text-sm font-medium mb-1">{"Title"}</label>
                                            <input
                                                type="text"
                                                required=true
                                                oninput={title_handler}
                                                class="transition-all transition-discrete ease-in-out duration-150
                                                       hover:-translate-y-1 text-middle placeholder:text-center 
                                                       border border-purple-600 rounded-lg px-1 
                                                       focus:outline-violet-700"
                                            />
                                        </div>
                                    </>
                                },
                                FormType::Comment => html! {}
                            }}

                            <div class="mb-4 gap-2">
                                <label class="block text-sm font-medium mb-1">{"Content"}</label>
                                <textarea
                                    required=true
                                    oninput={Callback::from(move |e: InputEvent| {
                                        let textarea: HtmlTextAreaElement = e.target_unchecked_into();
                                        content.set(textarea.value());
                                    })}
                                    class="transition-all transition-discrete ease-in-out duration-150
                                           hover:-translate-y-1 text-middle placeholder:text-center 
                                           border border-purple-600 rounded-lg h-32 p-2 
                                           focus:outline-violet-700 w-full"
                                />
                            </div>

                            <div class="mb-4 gap-2">
                                <label class="block text-sm font-medium mb-1">{"Image"}</label>
                                <input
                                    type="file"
                                    accept="image/*"
                                    onchange={on_image_change}
                                    class="block w-full text-sm text-purple-500
                                           file:mr-4 file:py-2 file:px-4
                                           file:rounded-full file:border-0
                                           file:text-sm file:font-semibold
                                           file:bg-purple-500 file:text-white
                                           hover:file:bg-purple-600"
                                />
                                {if let Some(url) = &*preview_url {
                                    html! { <BlobImage data={image_data.as_ref().unwrap().clone()} /> }
                                } else {
                                    html! {}
                                }}
                            </div>

                            <div class="flex justify-end gap-2">
                                <button
                                    type="button"
                                    onclick={close_modal.clone()}
                                    class="px-4 py-2 text-gray-600 hover:bg-gray-50 rounded-lg"
                                >
                                    {"Cancel"}
                                </button>
                                <button
                                    type="submit"
                                    class="px-4 py-2 bg-purple-500 hover:bg-purple-600 text-white font-medium rounded-lg transition-colors"
                                >
                                    {"Submit"}
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            }
        </>
    }
}