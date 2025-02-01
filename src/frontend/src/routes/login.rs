use gloo_storage::{LocalStorage, Storage};
use web_sys::console;
use yew_router::prelude::*;
use yew::{platform::spawn_local, prelude::*};

use crate::{libs::request::login_req::login_req, router::Route, ui::{button::Button, input::Input}};

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();

    let input_username = use_state(|| String::new());
    let on_username_change = {
        let input_username = input_username.clone();
        Callback::from(move |value: String| {
            input_username.set(value);
        })
    };

    let input_password = use_state(|| String::new());
    let on_password_change = {
        let input_password = input_password.clone();
        Callback::from(move |value: String| {
            input_password.set(value);
        })
    };

    let error = use_state(|| None::<String>);

    let on_submit = {
        let username = input_username.clone();
        let password = input_password.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let username = username.to_string();
            let password = password.to_string();
            let error = error.clone();
            let nav = navigator.clone();

            spawn_local(async move {
                match login_req(username, password).await {
                    Ok(token) => {
                        LocalStorage::set("token", token).unwrap();
                        error.set(None);
                        console::log_1(&"login success".into());
                        nav.push(&Route::Home)
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
            });
        })
    };

    html! {
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
            <div class="flex flex-col align-middle justify-center items-center">
                <div class="flex flex-col align-middle justify-center items-center gap-6 border-4 border-dashed border-purple-600 p-10 rounded-xl">
                    <h1 class="text-2xl pb-4">{ "Login" }</h1>

                    <Input 
                        value={(*input_username).clone()} 
                        on_change={on_username_change} 
                        placeholder="Enter A Username"
                    />

                    <Input 
                        value={(*input_password).clone()} 
                        on_change={on_password_change} 
                        placeholder="Enter A Password"
                    />

                    <Button 
                        label="Submit" 
                        on_click={on_submit}
                    />

                    <div class="flex flex-col gap-1">
                        <p class="text-xs text-purple-500">{"Don't have an account?"}</p>
                        <div class="w-full justify-center align-middle items-center flex flex-row">
                            <Link<Route> to={Route::Signup} classes="
                                transform transition-all transition-discrete
                                ease-in-out ease-in-out
                                w-min text-xs text-purple-400 
                                hover:-translate-y-0.5
                                hover:text-purple-200">
                                {"Signup"}
                            </Link<Route>>
                        </div>
                    </div>

                    {if let Some(e) = &*error {
                        html! { <p class="
                            text-red-500
                            transition-all
                            transition-discrete
                            ease-in-out
                            duration-150">{e}</p> }
                    } else {
                        html! {}
                    }}

                </div>

            </div>
        </div>
    }
}