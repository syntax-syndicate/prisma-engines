use js_sys::Array;
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
}

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
    pub fn get_arguments() {}
}

#[wasm_bindgen(js_name = "buildSchema")]
pub fn build_schema(datamodel_string: String) -> Schema {
    let mut schema = psl::validate(datamodel_string.into());

    schema.diagnostics.to_result().unwrap();

    let internal_data_model = prisma_models::convert(Arc::new(schema), "".into());
    let query_schema = schema_builder::build(internal_data_model, true);

    Schema::new(query_schema)
}
