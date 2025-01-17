use dioxus::prelude::*;
use ui::{Echo, Hero};

use web_sys::{window, HtmlElement};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn Home() -> Element {

    use_effect(move || {
        let window = window().expect("should have a window");
        let document = window.document().expect("should have a document");

        let adjust_button_position = {
            let document = document.clone();
            let window = window.clone();
            move || {
                if let Some(button) = document.get_element_by_id("fixed-button").and_then(|el| el.dyn_into::<HtmlElement>().ok()) {
                    let viewport_height = window.inner_height().unwrap().as_f64().unwrap_or(0.0);
                    let screen_height = window.screen().unwrap().height().unwrap() as f64;

                    if viewport_height < screen_height {
                        
                        let offset = screen_height - viewport_height;
                        button.style().set_property("bottom", &format!("{}px", offset)).unwrap();
                    } else {
                        
                        button.style().set_property("bottom", "0px").unwrap();
                    }
                }
            }
        };

        // registe resize event handler
        let closure = {
            let adjust_button_position = adjust_button_position.clone();
            Closure::wrap(Box::new(move || adjust_button_position()) as Box<dyn FnMut()>)
        };
        window
            .clone()
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
            .unwrap();

        adjust_button_position();
        // cleanup
        (move || drop(closure))()
    });

    rsx! {
        Echo {}
    }
}
