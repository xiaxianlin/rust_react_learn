use std::rc::Rc;

use host_config::ReactDomHostConfig;
use react_reconciler::Reconciler;
use renderer::Renderer;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::Node;

mod host_config;
mod renderer;
mod utils;

#[wasm_bindgen(js_name = createRoot)]
pub fn create_root(container: &JsValue) -> Renderer {
    set_panic_hook();
    let reconciler = Reconciler::new(Rc::new(ReactDomHostConfig));
    let node = match container.clone().dyn_into::<Node>() {
        Ok(node) => node,
        Err(_) => {
            panic!("container should be Node")
        }
    };
    let root = reconciler.create_container(Rc::new(node));
    let renderer = Renderer::new(root, reconciler);
    renderer
}
