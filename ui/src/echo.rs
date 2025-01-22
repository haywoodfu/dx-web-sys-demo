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
            style: "width:100%;",
            h4 { "ServerFn Echo" }
            input {
                id: "echo-input",
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = server::echo(event.value()).await.unwrap();
                    response.set(data);
                },
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
