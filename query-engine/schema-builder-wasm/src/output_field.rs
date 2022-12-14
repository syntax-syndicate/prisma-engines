use crate::input_field::InputField;
use js_sys::Array;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct OutputField {
    inner: schema::OutputFieldRef,
}

impl OutputField {
    pub fn new(inner: schema::OutputFieldRef) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
impl OutputField {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(js_name = "getArguments")]
    pub fn get_arguments(&self) -> Array {
        self.inner
            .arguments
            .iter()
            .map(|arg| JsValue::from(InputField::new(Arc::clone(arg))))
            .collect()
    }
}
