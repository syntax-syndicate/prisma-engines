use crate::functions::get_input_type;
use js_sys::Array;
use once_cell::sync::{Lazy, OnceCell};
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct InputField {
    inner: schema::InputFieldRef,
    fields: OnceCell<Array>,
}

impl InputField {
    pub fn new(inner: schema::InputFieldRef) -> Self {
        Self {
            inner,
            fields: OnceCell::new(),
        }
    }
}

#[wasm_bindgen]
impl InputField {
    #[wasm_bindgen(js_name=getName)]
    pub fn get_name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn is_required(&self) -> bool {
        self.inner.is_required
    }

    #[wasm_bindgen(js_name = "getFieldTypes")]
    pub fn get_field_types(&self) -> Array {
        self.fields
            .get_or_init(|| self.inner.field_types.iter().map(get_input_type).collect())
            .to_owned()
    }
}
