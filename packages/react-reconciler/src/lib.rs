use std::{any::Any, cell::RefCell, rc::Rc};

use fiber::{FiberNode, FiberRootNode, StateNode};
use update_queue::{create_update, enqueue_update};
use wasm_bindgen::prelude::*;
use work_loop::WorkLoop;
use work_tags::WorkTag;

mod begin_work;
mod child_fiber;
mod complete_work;
pub mod fiber;
mod fiber_flags;
mod update_queue;
mod work_loop;
mod work_tags;

pub trait HostConfig {
    fn create_text_instance(&self, content: String) -> Rc<dyn Any>;
    fn create_instance(&self, _type: String) -> Rc<dyn Any>;
    fn append_initial_child(&self, parent: Rc<dyn Any>, child: Rc<dyn Any>);
    fn append_child_to_container(&self, child: Rc<dyn Any>, parent: Rc<dyn Any>);
}

pub struct Reconciler {
    host_config: Rc<dyn HostConfig>,
}

impl Reconciler {
    pub fn new(host_config: Rc<dyn HostConfig>) -> Self {
        Reconciler { host_config }
    }

    pub fn create_container(&self, container: &JsValue) -> Rc<RefCell<FiberRootNode>> {
        let host_root_fiber = Rc::new(RefCell::new(FiberNode::new(WorkTag::HostRoot, None, None)));
        host_root_fiber
            .clone()
            .borrow_mut()
            .initialize_update_queue();

        let root = Rc::new(RefCell::new(FiberRootNode::new(
            Rc::new(container.clone()),
            host_root_fiber.clone(),
        )));
        let r1 = root.clone();
        host_root_fiber.borrow_mut().state_node = Some(Rc::new(StateNode::FiberRootNode(r1)));
        root.clone()
    }

    pub fn update_container(&self, element: Rc<JsValue>, root: Rc<RefCell<FiberRootNode>>) {
        let host_root_fiber = Rc::clone(&root).borrow().current.clone();
        let update = create_update(element);
        enqueue_update(host_root_fiber.borrow(), update);

        let mut work_loop = WorkLoop::new(self.host_config.clone());
        work_loop.schedule_update_on_fiber(host_root_fiber);
    }
}
