use std::{cell::RefCell, rc::Rc};

use shared::{derive_from_js_value, log};
use wasm_bindgen::JsValue;

use crate::{
    child_fiber::{mount_child_fibers, reconcile_child_fibers},
    fiber::FiberNode,
    update_queue::process_update_queue,
    work_tags::WorkTag,
};

pub fn begin_work(work_in_progress: Rc<RefCell<FiberNode>>) -> Option<Rc<RefCell<FiberNode>>> {
    let tag = work_in_progress.clone().borrow().tag.clone();

    return match tag {
        WorkTag::FunctionComponent => None,
        WorkTag::HostRoot => update_host_root(work_in_progress.clone()),
        WorkTag::HostComponent => update_host_component(work_in_progress.clone()),
        WorkTag::HostText => None,
    };
}

pub fn update_host_root(
    work_in_progress: Rc<RefCell<FiberNode>>,
) -> Option<Rc<RefCell<FiberNode>>> {
    process_update_queue(work_in_progress.clone());
    let next_children = work_in_progress.clone().borrow().memoized_state.clone();
    log!("tag {:?}", next_children);
    reconcile_children(work_in_progress.clone(), next_children);
    work_in_progress.clone().borrow().child.clone()
}

pub fn update_host_component(
    work_in_progress: Rc<RefCell<FiberNode>>,
) -> Option<Rc<RefCell<FiberNode>>> {
    let work_in_progress = Rc::clone(&work_in_progress);

    let next_children = {
        let ref_fiber_node = work_in_progress.borrow();
        derive_from_js_value(ref_fiber_node.pending_props.clone().unwrap(), "children")
    };

    {
        reconcile_children(work_in_progress.clone(), next_children);
    }
    work_in_progress.clone().borrow().child.clone()
}

pub fn reconcile_children(work_in_progress: Rc<RefCell<FiberNode>>, children: Option<Rc<JsValue>>) {
    let work_in_progress = Rc::clone(&work_in_progress);
    let current = { work_in_progress.borrow().alternate.clone() };

    work_in_progress.borrow_mut().child = if current.is_some() {
        reconcile_child_fibers(work_in_progress.clone(), current.clone(), children)
    } else {
        mount_child_fibers(work_in_progress.clone(), None, children)
    };
}
