mod enum_type;
mod functions;
mod input_field;
mod input_object_type;
mod object_type;
mod output_field;
mod scalar_type;
mod schema;

use crate::schema::Schema;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "buildSchema")]
pub fn build_schema(datamodel_string: String) -> Schema {
    let mut schema = psl::validate(datamodel_string.into());

    schema.diagnostics.to_result().unwrap();

    let internal_data_model = prisma_models::convert(Arc::new(schema), "".into());
    let query_schema = schema_builder::build(internal_data_model, true);

    Schema::new(query_schema)
}
