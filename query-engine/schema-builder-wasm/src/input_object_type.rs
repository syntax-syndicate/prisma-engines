use crate::input_field::InputField;
use js_sys::Array;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct InputObjectType {
    inner: schema::InputObjectTypeStrongRef,
}

impl InputObjectType {
    pub fn new(inner: schema::InputObjectTypeStrongRef) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
impl InputObjectType {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.identifier.name().into()
    }

    #[wasm_bindgen(js_name = getFields)]
    pub fn get_fields(&self) -> Array {
        self.inner
            .get_fields()
            .iter()
            .map(|f| JsValue::from(InputField::new(Arc::clone(f))))
            .collect()
    }

    #[wasm_bindgen(js_name = findField)]
    pub fn find_field(&self, name: String) -> Option<InputField> {
        self.inner.find_field(name).map(InputField::new)
    }
}
