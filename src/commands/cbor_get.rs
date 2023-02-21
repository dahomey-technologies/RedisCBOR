use crate::redis_cbor_type::get_value;
use cbor_data::{CborBuilder, Writer};
use cborpath::CborPath;
use redis_module::{Context, NextArg, RedisResult, RedisString, RedisValue, RedisError};

pub fn cbor_get(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);

    let key = args.next_arg()?;

    let cbor_path = if let Ok(cbor_path) = args.next_arg() {
        let Ok(cbor_path) = CborPath::from_bytes(cbor_path.as_slice()) else {
            return Err(RedisError::Str("ERR Invalid CBOR path"));
        };
        cbor_path
    } else {
        CborPath::root()
    };

    let key = ctx.open_key(&key);
    let value = get_value(&key)?;

    match value {
        Some(value) => {
            let results = cbor_path.read(value);
            let output = CborBuilder::new().write_array(None, |builder| {
                for result in results {
                    builder.write_item(result);
                }
            });
            Ok(RedisValue::StringBuffer(output.into_vec()))
        },
        None => Ok(RedisValue::Null),
    }
}
