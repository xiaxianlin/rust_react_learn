use std::{any::Any, cell::RefCell, rc::Rc};

use shared::{derive_from_js_value, log};
use web_sys::js_sys::Object;

use crate::{
    fiber::{FiberNode, StateNode},
    fiber_flags::Flags,
    work_tags::WorkTag,
    HostConfig,
};

pub struct CompleteWork {
    pub host_config: Rc<dyn HostConfig>,
}

impl CompleteWork {
    pub(crate) fn new(host_config: Rc<dyn HostConfig>) -> Self {
        Self { host_config }
    }

    fn append_all_children(&self, parent: Rc<dyn Any>, work_in_progress: Rc<RefCell<FiberNode>>) {
        let work_in_progress = work_in_progress.clone();
        let mut node = work_in_progress.borrow().child.clone();
        while node.is_some() {
            let node_unwrap = node.clone().unwrap();
            let n = node_unwrap.clone();
            if n.borrow().tag == WorkTag::HostComponent || n.borrow().tag == WorkTag::HostText {
                self.host_config.append_initial_child(
                    parent.clone(),
                    FiberNode::derive_state_node(node.clone().unwrap()).unwrap(),
                )
            } else if n.borrow().child.is_some() {
                let n = node_unwrap.clone();
                {
                    let borrowed = n.borrow_mut();
                    borrowed
                        .child
                        .as_ref()
                        .unwrap()
                        .clone()
                        .borrow_mut()
                        ._return = Some(node_unwrap.clone());
                }
                node = node_unwrap.clone().borrow().child.clone();
                continue;
            }

            if Rc::ptr_eq(&node_unwrap, &work_in_progress) {
                return;
            }

            while node
                .clone()
                .unwrap()
                .clone()
                .borrow()
                .sibling
                .clone()
                .is_none()
            {
                let node_cloned = node.clone().unwrap().clone();
                if node_cloned.borrow()._return.is_none()
                    || Rc::ptr_eq(
                        &node_cloned.borrow()._return.as_ref().unwrap(),
                        &work_in_progress,
                    )
                {
                    return;
                }
                node = node_cloned.borrow()._return.clone();
            }

            {
                node.clone()
                    .unwrap()
                    .borrow()
                    .sibling
                    .clone()
                    .unwrap()
                    .clone()
                    .borrow_mut()
                    ._return = node_unwrap.borrow()._return.clone();
            }

            node = node.clone().unwrap().borrow().sibling.clone();
        }
    }

    fn bubble_properties(&self, complete_work: Rc<RefCell<FiberNode>>) {
        let mut subtree_flags = Flags::NoFlags;
        {
            let mut child = complete_work.clone().borrow().child.clone();
            while child.is_some() {
                let child_rc = child.clone().unwrap().clone();
                {
                    let child_borrowed = child_rc.borrow();
                    subtree_flags |= child_borrowed.subtree_flags.clone();
                    subtree_flags |= child_borrowed.flags.clone();
                }
                {
                    child_rc.borrow_mut()._return = Some(complete_work.clone());
                }
                child = child_rc.borrow().sibling.clone();
            }
        }
        complete_work.clone().borrow_mut().subtree_flags |= subtree_flags.clone();
    }

    fn mark_update(fiber: Rc<RefCell<FiberNode>>) {
        fiber.borrow_mut().flags |= Flags::Update;
    }

    pub fn complete_work(
        &self,
        work_in_progress: Rc<RefCell<FiberNode>>,
    ) -> Option<Rc<RefCell<FiberNode>>> {
        let work_in_progress_cloned = work_in_progress.clone();
        let new_props = { work_in_progress_cloned.borrow().pending_props.clone() };
        let current = { work_in_progress_cloned.borrow().alternate.clone() };
        let tag = { work_in_progress_cloned.borrow().tag.clone() };

        match tag {
            WorkTag::FunctionComponent => {
                self.bubble_properties(work_in_progress.clone());
                None
            }
            WorkTag::HostRoot => {
                self.bubble_properties(work_in_progress.clone());
                None
            }
            WorkTag::HostComponent => {
                if current.is_some() && work_in_progress_cloned.borrow().state_node.is_some() {
                    log!("update properties")
                } else {
                    let instance = self.host_config.create_instance(
                        work_in_progress
                            .clone()
                            .borrow()
                            ._type
                            .as_ref()
                            .as_string()
                            .unwrap(),
                    );
                    self.append_all_children(instance.clone(), work_in_progress.clone());
                    work_in_progress.clone().borrow_mut().state_node =
                        Some(Rc::new(StateNode::Element(instance.clone())));
                }

                self.bubble_properties(work_in_progress.clone());
                None
            }
            WorkTag::HostText => {
                if current.is_some() && work_in_progress_cloned.borrow().state_node.is_some() {
                    let old_text = derive_from_js_value(
                        &current.clone().unwrap().clone().borrow().memoized_props,
                        "content",
                    );
                    let new_text = derive_from_js_value(&new_props, "content");
                    if !Object::is(&old_text, &new_text) {
                        CompleteWork::mark_update(work_in_progress.clone());
                    }
                } else {
                    let text_instance = self.host_config.create_text_instance(
                        derive_from_js_value(&new_props, "content")
                            .as_string()
                            .unwrap(),
                    );
                    work_in_progress.clone().borrow_mut().state_node =
                        Some(Rc::new(StateNode::Element(text_instance.clone())));
                }
                self.bubble_properties(work_in_progress.clone());
                None
            }
        }
    }
}
