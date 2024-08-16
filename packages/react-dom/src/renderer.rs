use std::{cell::RefCell, rc::Rc};

use react_reconciler::{fiber::FiberRootNode, Reconciler};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Renderer {
    root: Rc<RefCell<FiberRootNode>>,
    reconciler: Reconciler,
}

impl Renderer {
    pub fn new(root: Rc<RefCell<FiberRootNode>>, reconciler: Reconciler) -> Self {
        Self { root, reconciler }
    }
}

#[wasm_bindgen]
impl Renderer {
    pub fn render(&self, element: &JsValue) -> JsValue {
        self.reconciler
            .update_container(element.clone(), self.root.clone())
    }
}
