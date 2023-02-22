use crate::redis_cbor_type::REDIS_CBOR_TYPE;

mod redis_cbor_type;
mod commands;
mod util;

pub const MODULE_NAME: &str = "ReCBOR";

#[macro_use]
extern crate redis_module;

redis_module! {
    name: MODULE_NAME,
    version: 1,
    data_types: [REDIS_CBOR_TYPE],
    commands: [
        ["cbor.arrappend", commands::cbor_arr_append, "write deny-oom", 1, 1, 1],
        ["cbor.debug", commands::cbor_debug, "readonly", 1, 1, 1],
        ["cbor.get", commands::cbor_get, "readonly", 1, 1, 1],
        ["cbor.set", commands::cbor_set, "write deny-oom", 1, 1, 1],
    ],
}
