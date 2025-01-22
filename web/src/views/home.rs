use dioxus::prelude::*;
use dioxus_elements::style;
use ui::{Echo, Hero};
use dioxus_logger::tracing::debug;
use web_sys::{window, HtmlElement};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use dioxus::prelude::KeyboardEvent;

#[component]
pub fn Home() -> Element {
    
    let mut closure_state = use_signal(|| None::<Closure<dyn FnMut()>>);
    use_effect(move || {
        debug!("Effect triggered for Home");
        let window = window().expect("should have a window");
        let document = window.document().expect("should have a document");
        let adjust_button_position = {
            let document = document.clone();
            let window = window.clone();
            move || {
                if let Some(button) = document.get_element_by_id("fixed-button").and_then(|el| el.dyn_into::<HtmlElement>().ok()) {
                    
                    let viewport_height = window.inner_height().unwrap().as_f64().unwrap_or(0.0);
                    let screen_height = window.screen().unwrap().height().unwrap() as f64;
                    let v_height = window.visual_viewport().unwrap().height();
                    debug!("Adjusting button position viewport_height:{} screen_height:{} v_height:{}", viewport_height, screen_height, v_height);
                    if viewport_height < screen_height {
                        
                        let offset = screen_height - viewport_height;
                        button.style().set_property("bottom", &format!("{}px", offset)).unwrap();
                    } else {
                        
                        button.style().set_property("bottom", "0px").unwrap();
                    }
                }
            }
        };


        let callback = Closure::wrap(Box::new(move || {
            debug!("resize event");
            adjust_button_position();
        }) as Box<dyn FnMut()>);

        closure_state.set(Some(callback));

        if let Some(closure) = closure_state.read().as_ref() {
            window
                .clone()
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
        }
        // || {
        //     debug!("cleanup Home");
        // }
    });

    use_drop({

        || {
            debug!("use_drop Home");
        }
    });
    // use_effect({
    //     // let window = window.clone();
    //      || {
        

    //     ( || {
    //         debug!("cleanup");
    //     })()
    // }});


    

    
    

    // window
    //             .clone()
    //             .add_event_listener_with_callback("resize", closure_state.read().as_ref().unchecked_ref())
    //             .unwrap();

    //         adjust_button_position();

    // use_effect( {
    //     let window = window.clone();
    //     move || {
    //         // registe resize event handler
    //         let adjust_button_position_clone = adjust_button_position.clone();
    //         let closure = Closure::wrap(Box::new(move || {
    //             debug!("resize event");
    //             adjust_button_position_clone();
    //         }
    //         ) as Box<dyn FnMut()>);

    //         closure_state.set(Some(closure));

    //         if let Some(closure) = closure_state.read().as_ref() {
    //             window
    //                 .clone()
    //                 .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
    //                 .unwrap();
    //         }

    //         adjust_button_position();

    //     // let closure_opt = Some(closure);
        
    //     // cleanup
    //     // (move || {
    //     //     debug!("cleanup");
    //     //     if let Some(closure_val) = closure_opt {
    //     //         window
    //     //             .remove_event_listener_with_callback("resize", closure_val.as_ref().unchecked_ref())
    //     //             .unwrap();
    //     //         drop(closure_val);
    //     //     }
    //     // })()
    // }});
    // use_drop({
    //     let window = window.clone();
    //     let closure_state = closure_state.clone();
    //     move || {
    //         debug!("use_drop");
    //         if let Some(closure) = *closure_state.read() {
    //             window
    //                 .clone()
    //                 .remove_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
    //                 .unwrap();
    //             drop(closure);
    //         }
    //     }
    // });

    // use_effect({
    //     // let window = window.clone();
    //      || {
    //     let window = window().expect("should have a window");
    //     // let document = window.document().expect("should have a document");
    //     let callback = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
    //         let key = e.key();
    //         let key_str = key.as_str();
    //         debug!("key pressed: {}", key_str);
    //         match key_str {
    //             "ArrowLeft" => {
    //               // Do something here
    //               debug!("ArrowLeft");
    //             }
    //             "ArrowRight" => {
    //               // Do something here
    //               debug!("ArrowRight");
    //             }
    //             _ => {}
    //         }
    //     }) as Box<dyn FnMut(_)>);

    //     window.clone().add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())
    //         .unwrap();

    //     // ( || {
    //     //     debug!("cleanup");
    //     //     window.clone().remove_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())
    //     //         .unwrap();
    //     // })()
    // }});

    rsx! {
        form {
            style:  "position: relative; display: flex; flex-direction: column; align-items: start; justify-content: start; height: 100vh; font-family: Arial, sans-serif;",
            // onresize: |event| {
            //     debug!("resize event: {:?}", event);
            //     let window = window().expect("should have a window");
            //     let document = window.document().expect("should have a document");
            //     if let Some(button) = document.get_element_by_id("fixed-button").and_then(|el| el.dyn_into::<HtmlElement>().ok()) {
                    
            //         let viewport_height = window.inner_height().unwrap().as_f64().unwrap_or(0.0);
            //         let screen_height = window.screen().unwrap().height().unwrap() as f64;
            //         debug!("Adjusting button position viewport_height:{} screen_height:{}", viewport_height, screen_height);
            //         if viewport_height < screen_height {
                        
            //             let offset = screen_height - viewport_height;
            //             button.style().set_property("bottom", &format!("{}px", offset)).unwrap();
            //         } else {
                        
            //             button.style().set_property("bottom", "0px").unwrap();
            //         }
            //     }
            
            // },
            Echo {}
            button {
                id: "fixed-button",
                style: "position: fixed; bottom: 0; left: 0;width:200px;height:50px; background-color: #f44336; color: white; padding: 15px 32px; text-align: center; text-decoration: none; display: inline-block; font-size: 16px;",
                "Echo"
            }
        }
    }
}
