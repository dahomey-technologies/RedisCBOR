use crate::{
    redis_cbor_type::{get_writable_value, set_value},
    util::apply_changes,
};
use cbor_data::Cbor;
use cborpath::CborPath;
use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString, RedisValue, REDIS_OK};

#[derive(Debug, PartialEq, Eq)]
pub enum SetOptions {
    NotExists,
    AlreadyExists,
    None,
}

pub fn cbor_set(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);

    let key_name = args.next_arg()?;
    let cbor_path = args.next_arg()?;
    let value = args.next_arg()?;

    let Ok(cbor_path) = CborPath::from_bytes(cbor_path.as_slice()) else {
        return Err(RedisError::Str("ERR Invalid CBOR path"));
    };

    let Ok(value) = Cbor::checked(value.as_slice()) else {
        return Err(RedisError::Str("ERR Invalid CBOR value"));
    };

    let mut set_options = SetOptions::None;

    for s in args {
        match s.try_as_str()? {
            arg if arg.eq_ignore_ascii_case("NX") && set_options == SetOptions::None => {
                set_options = SetOptions::NotExists
            }
            arg if arg.eq_ignore_ascii_case("XX") && set_options == SetOptions::None => {
                set_options = SetOptions::AlreadyExists
            }
            _ => return Err(RedisError::Str("ERR syntax error")),
        };
    }

    let key = ctx.open_key_writable(&key_name);
    let existing = get_writable_value(&key)?;

    match (existing, set_options) {
        (Some(_), SetOptions::NotExists) => Ok(RedisValue::Null),
        (None, SetOptions::AlreadyExists) => Ok(RedisValue::Null),
        (Some(existing), _) => {
            let new_value = cbor_path.set(existing, value);
            set_value(&key, new_value.into_owned())?;
            apply_changes(ctx, "cbor.set", &key_name)?;
            REDIS_OK
        }
        (None, _) => {
            if cbor_path.is_root() {
                set_value(&key, value.to_owned())?;
                apply_changes(ctx, "cbor.set", &key_name)?;
                REDIS_OK
            } else {
                Err(RedisError::Str(
                    "ERR new CBOR documents must be created with a root path",
                ))
            }
        }
    }
}
