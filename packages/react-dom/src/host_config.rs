use std::{any::Any, rc::Rc};

use react_reconciler::HostConfig;
use web_sys::{window, Element, Text};

pub struct ReactDomHostConfig;

impl HostConfig for ReactDomHostConfig {
    fn create_text_instance(&self, content: String) -> Rc<dyn Any> {
        let window = window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        Rc::new(document.create_text_node(content.as_str()))
    }

    fn create_instance(&self, _type: String) -> Rc<dyn Any> {
        let window = window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        match document.create_element(_type.as_ref()) {
            Ok(element) => Rc::new(element),
            Err(_) => todo!(),
        }
    }

    fn append_initial_child(&self, parent: Rc<dyn Any>, child: Rc<dyn Any>) {
        let parent = parent.clone().downcast::<Element>().unwrap();
        let child = child.downcast::<Text>().unwrap();
        match parent.append_child(&child) {
            Ok(_) => {
                println!(
                    "append_initial_child successfully ele {:?} {:?}",
                    parent, child
                );
            }
            Err(_) => todo!(),
        }
    }

    fn append_child_to_container(&self, _child: Rc<dyn Any>, _parent: Rc<dyn Any>) {
        todo!()
    }
}
