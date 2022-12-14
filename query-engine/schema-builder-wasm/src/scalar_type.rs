use crate::output_field::OutputField;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ScalarType {
    name: String,
}

impl ScalarType {
    pub fn new(inner: &schema::ScalarType) -> Self {
        Self {
            name: format!("{}", inner),
        }
    }
}

#[wasm_bindgen]
impl ScalarType {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
