use dioxus::prelude::*;
use global_attributes::id;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

/// Echo component that demonstrates fullstack server functions.
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                id: "echo-input",
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = server::echo(event.value()).await.unwrap();
                    response.set(data);
                },
            }
            button {
                id: "fixed-button",
                style: "position: fixed; bottom: 0; left: 0;width:200px;height:50px; background-color: #f44336; color: white; padding: 15px 32px; text-align: center; text-decoration: none; display: inline-block; font-size: 16px;",
                "Echo"
            }
            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}
