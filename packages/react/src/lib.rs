use current_dispatcher::CURRENT_DISPATCHER;
use js_sys::{Object, Reflect, JSON};
use shared::REACT_ELEMENT_TYPE;
use wasm_bindgen::prelude::*;

pub mod current_dispatcher;

fn resolve_key(value: &JsValue) -> JsValue {
    if value.is_undefined() {
        JsValue::null()
    } else if value.is_string() {
        value.clone()
    } else {
        JSON::stringify(value).unwrap().into()
    }
}

fn resolve_ref(val: &JsValue) -> JsValue {
    if val.is_undefined() {
        JsValue::null()
    } else {
        val.clone()
    }
}

#[wasm_bindgen(js_name = jsxDEV)]
pub fn jsx_dev(_type: &JsValue, config: &JsValue, key: &JsValue) -> JsValue {
    let react_element = Object::new();
    let mut _ref = JsValue::null();
    let mut key = resolve_key(key);
    Reflect::set(
        &react_element,
        &"$$typeof".into(),
        &JsValue::from_str(REACT_ELEMENT_TYPE),
    )
    .expect("$$typeof panic");
    Reflect::set(&react_element, &"type".into(), _type).expect("_type panic");

    let props = Object::new();
    if let Some(conf) = config.dyn_ref::<Object>() {
        for prop in Object::keys(conf) {
            let val = Reflect::get(conf, &prop);
            match prop.as_string() {
                None => {}
                Some(k) => {
                    if k == "ref" && val.is_ok() {
                        _ref = resolve_ref(&val.unwrap());
                    } else if k == "key" && val.is_ok() {
                        key = resolve_key(&val.unwrap());
                    } else {
                        Reflect::set(&props, &JsValue::from(k), &val.unwrap())
                            .expect("props panic");
                    }
                }
            }
        }
        Reflect::set(&react_element, &"props".into(), &props).expect("props panic");
    } else {
        if config.is_object() {
            Reflect::set(&react_element, &"props".into(), &config).expect("props panic");
        } else {
            Reflect::set(&react_element, &"props".into(), &props).expect("props panic");
        }
    }

    Reflect::set(&react_element, &"ref".into(), &_ref).expect("ref panic");
    Reflect::set(&react_element, &"key".into(), &key).expect("key panic");

    react_element.into()
}

#[wasm_bindgen(js_name = createElement)]
pub fn create_element(_type: &JsValue, config: &JsValue, key: &JsValue) -> JsValue {
    jsx_dev(_type, config, key)
}

#[wasm_bindgen(js_name = isValidElement)]
pub fn is_valid_element(object: &JsValue) -> bool {
    object.is_object()
        && !object.is_null()
        && Reflect::get(&object, &"$$typeof".into())
            .unwrap_or("".into())
            .as_string()
            .unwrap_or("".into())
            .as_str()
            == REACT_ELEMENT_TYPE
}

#[wasm_bindgen(js_name = useState)]
pub unsafe fn use_state(initial_state: &JsValue) -> Result<JsValue, JsValue> {
    let use_state = &CURRENT_DISPATCHER.current.as_ref().unwrap().use_state;
    use_state.call1(&JsValue::null(), initial_state)
}
