use std::{any::Any, cell::RefCell, rc::Rc};

use fiber::{FiberNode, FiberRootNode, StateNode};
use fiber_hooks::WORK_LOOP;
use update_queue::{create_update, create_update_queue, enqueue_update};
use wasm_bindgen::prelude::*;
use work_loop::WorkLoop;
use work_tags::WorkTag;

mod begin_work;
mod child_fiber;
mod commit_work;
mod complete_work;
pub mod fiber;
mod fiber_flags;
mod fiber_hooks;
mod update_queue;
mod work_loop;
mod work_tags;

pub trait HostConfig {
    fn create_text_instance(&self, content: String) -> Rc<dyn Any>;
    fn create_instance(&self, _type: String) -> Rc<dyn Any>;
    fn append_initial_child(&self, parent: Rc<dyn Any>, child: Rc<dyn Any>);
    fn append_child_to_container(&self, child: Rc<dyn Any>, parent: Rc<dyn Any>);
    fn remove_child(&self, child: Rc<dyn Any>, container: Rc<dyn Any>);
    fn commit_text_update(&self, text_instance: Rc<dyn Any>, content: String);
}

pub struct Reconciler {
    host_config: Rc<dyn HostConfig>,
}

impl Reconciler {
    pub fn new(host_config: Rc<dyn HostConfig>) -> Self {
        Reconciler { host_config }
    }

    pub fn create_container(&self, container: Rc<dyn Any>) -> Rc<RefCell<FiberRootNode>> {
        let host_root_fiber = Rc::new(RefCell::new(FiberNode::new(
            WorkTag::HostRoot,
            JsValue::null(),
            JsValue::null(),
        )));
        host_root_fiber.clone().borrow_mut().update_queue = Some(create_update_queue());

        let root = Rc::new(RefCell::new(FiberRootNode::new(
            container.clone(),
            host_root_fiber.clone(),
        )));
        let r1 = root.clone();
        host_root_fiber.borrow_mut().state_node = Some(Rc::new(StateNode::FiberRootNode(r1)));
        root.clone()
    }

    pub fn update_container(&self, element: JsValue, root: Rc<RefCell<FiberRootNode>>) -> JsValue {
        let host_root_fiber = Rc::clone(&root).borrow().current.clone();
        let update = create_update(element.clone());
        enqueue_update(
            host_root_fiber.borrow().update_queue.clone().unwrap(),
            update,
        );

        let work_loop = Rc::new(RefCell::new(WorkLoop::new(self.host_config.clone())));
        unsafe {
            WORK_LOOP = Some(work_loop.clone());
        }
        work_loop
            .clone()
            .borrow()
            .schedule_update_on_fiber(host_root_fiber);
        element.clone()
    }
}
