use crate::enum_type::EnumType;
use crate::input_object_type::InputObjectType;
use crate::scalar_type::ScalarType;
use js_sys::Array;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

pub fn get_input_type(input_type: &schema::InputType) -> JsValue {
    match input_type {
        schema::InputType::Object(inner) => InputObjectType::new(inner.upgrade().unwrap()).into(),
        schema::InputType::Enum(inner) => EnumType::new(inner.upgrade().unwrap()).into(),
        schema::InputType::Scalar(inner) => ScalarType::new(inner).into(),
        _ => JsValue::UNDEFINED,
    }
}
