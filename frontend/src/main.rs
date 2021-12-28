use yew::prelude::*;

mod api;
mod components;

mod views;

use views::login::Login;

struct UserInfo {
    pub auth: String,
    pub loading: bool,
    pub error: Option<String>,
}

#[function_component(App)]
fn app() -> Html {
    let user = use_state(|| UserInfo {
        auth: "".to_string(),
        loading: false,
        error: None,
    });

    let on_login = {
        let user = user.clone();

        Callback::from(move |(username, password): (String, String)| {
            let user = user.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let result = api::login(&username, &password).await;

                match result {
                    Err(_) => {
                        user.set(UserInfo {
                            auth: "".to_string(),
                            loading: false,
                            error: Some("Login request failed to send.".to_string()),
                        });
                    }
                    Ok(response) => {
                        if response.status() == 200 {
                            let auth: String = api::build_auth_header(&username, &password);
                            user.set(UserInfo {
                                auth,
                                loading: false,
                                error: None,
                            });
                        } else {
                            let message = response.text().await.unwrap();
                            user.set(UserInfo {
                                auth: "".to_string(),
                                loading: false,
                                error: Some(message),
                            });
                        }
                    }
                }
            });
        })
    };

    html! {
        <div class="flex flex-col h-screen">
            <Login
                loading={user.loading}
                error={user.error.clone()}
                {on_login}
            />
        </div>
    }
}

fn main() {
    // set up wasm-logger, to convert Rust's log into console-calls.
    wasm_logger::init(wasm_logger::Config::default());

    // Start the Yew app.
    yew::start_app::<App>();
}
