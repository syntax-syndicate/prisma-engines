use crate::error::ApiError;
use request_handlers::dmmf;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

use generated;

#[derive(serde::Serialize, Clone, Copy)]
#[napi(object)]
pub struct Version {
    pub commit: &'static str,
    pub version: &'static str,
}

#[napi]
pub fn version() -> Version {
    Version {
        commit: generated::GIT_HASH,
        version: generated::CARGO_PKG_VERSION,
    }
}

#[wasm_bindgen(js_name = debugPanic)]
pub fn dmmf(datamodel_string: String) -> Result<String, JsError> {
    let validate_schema = psl::validate(datamodel_string.into());

    validate_schema
        .diagnostics
        .to_result()
        .map_err(|errors| ApiError::conversion(errors, schema.db.source()))
        .map_err(|e| JsError::new(&e))?;

    let query_schema = query_core::schema::build(Arc::new(validate_schema), true);
    let dmmf = dmmf::render_dmmf(&query_schema);

    let dmmf_str = serde_json::to_string(&dmmf).unwrap();
    Ok(dmmf_str)
}

/// Trigger a panic inside the wasm module. This is only useful in development for testing panic
/// handling.
#[wasm_bindgen(js_name = debugPanic)]
pub fn debug_panic(panic_message: Option<String>) {
    let user_facing = user_facing_errors::Error::from_panic_payload(Box::new(
        panic_message.unwrap_or_else(|| "query-engine-node-api debug panic".to_string()),
    ));
    let message = serde_json::to_string(&user_facing).unwrap();
    panic!("{}", message);
}
