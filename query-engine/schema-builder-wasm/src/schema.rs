use crate::object_type::ObjectType;
use crate::output_field::OutputField;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Schema {
    inner: schema::QuerySchema,
}

impl Schema {
    pub fn new(inner: schema::QuerySchema) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
impl Schema {
    #[wasm_bindgen]
    pub fn query(&self) -> ObjectType {
        ObjectType::new(Arc::clone(&self.inner.query()))
    }

    #[wasm_bindgen]
    pub fn mutation(&self) -> ObjectType {
        ObjectType::new(Arc::clone(&self.inner.mutation()))
    }

    #[wasm_bindgen(js_name=findQueryField)]
    pub fn find_query_field(&self, name: String) -> Option<OutputField> {
        self.inner.find_query_field(name).map(OutputField::new)
    }

    #[wasm_bindgen(js_name=findMutationField)]
    pub fn find_mutation_field(&self, name: String) -> Option<OutputField> {
        self.inner.find_mutation_field(name).map(OutputField::new)
    }
}
