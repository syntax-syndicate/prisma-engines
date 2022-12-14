use crate::output_field::OutputField;
use js_sys::Array;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ObjectType {
    inner: schema::ObjectTypeStrongRef,
}

impl ObjectType {
    pub fn new(inner: schema::ObjectTypeStrongRef) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
impl ObjectType {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.identifier().name().into()
    }

    #[wasm_bindgen(js_name = "getFields")]
    pub fn get_fields(&self) -> Array {
        self.inner
            .get_fields()
            .into_iter()
            .map(|f| JsValue::from(OutputField::new(Arc::clone(&f))))
            .collect()
    }
}
