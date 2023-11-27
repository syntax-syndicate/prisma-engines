//! Query Engine Driver Adapters
//! This crate is responsible for defining a `quaint::Connector` implementation that uses functions
//! exposed by client connectors via either `napi-rs` (on native targets) or `wasm_bindgen` / `js_sys` (on Wasm targets).
//!
//! A driver adapter is an object defined in javascript that uses a driver
//! (ex. '@planetscale/database') to provide a similar implementation of that of a `quaint::Connector`. i.e. the ability to query and execute SQL
//! plus some transformation of types to adhere to what a `quaint::Value` expresses.
//!

pub(crate) mod conversion;
pub(crate) mod error;
pub(crate) mod queryable;
pub(crate) mod send_future;
pub(crate) mod types;

#[cfg(not(target_arch = "wasm32"))]
pub mod napi;

#[cfg(not(target_arch = "wasm32"))]
pub use napi::*;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

#[cfg(target_arch = "wasm32")]
mod arch {
    pub(crate) use crate::JsObjectExtern as JsObject;
    pub(crate) use js_sys::JsString;
    use tsify::Tsify;
    use wasm_bindgen::JsValue;

    pub(crate) fn get_named_property<T>(object: &JsObject, name: &str) -> JsResult<T>
    where
        T: From<JsValue>,
    {
        // object.get("queryRaw".into())?
        Ok(object.get(name.into())?.into())
    }

    pub(crate) fn to_rust_str(value: JsString) -> JsResult<String> {
        Ok(value.into())
    }

    pub(crate) fn from_js<C>(value: JsValue) -> C
    where
        C: Tsify + serde::de::DeserializeOwned,
    {
        C::from_js(value).unwrap()
    }

    pub(crate) type JsResult<T> = core::result::Result<T, JsValue>;
}

#[cfg(not(target_arch = "wasm32"))]
mod arch {
    use napi::bindgen_prelude::FromNapiValue;
    pub(crate) use napi::{JsObject, JsString};

    pub(crate) fn get_named_property<T>(object: &JsObject, name: &str) -> JsResult<T>
    where
        T: FromNapiValue,
    {
        object.get_named_property(name).into()
    }

    pub(crate) fn to_rust_str(value: JsString) -> JsResult<String> {
        Ok(value.into_utf8()?.as_str()?.to_string())
    }

    pub(crate) fn from_js<C>(value: C) -> C {
        value
    }

    pub(crate) type JsResult<T> = napi::Result<T>;
}

pub(crate) use arch::*;
