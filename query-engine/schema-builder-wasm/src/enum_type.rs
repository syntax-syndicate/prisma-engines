use crate::output_field::OutputField;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct EnumType {
    inner: schema::EnumTypeRef,
}

impl EnumType {
    pub fn new(inner: schema::EnumTypeRef) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
impl EnumType {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.identifier().name().into()
    }
}
