use crate::util::{CborKey, CborOwnedExt};
use redis_module::{Context, NextArg, RedisError, RedisResult, RedisString};

///
/// CBOT.DEBUG <subcommand & arguments>
///
/// subcommands:
/// MEMORY <key> [path]
/// HELP
///
pub fn cbor_debug(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);

    match args.next_str()?.to_uppercase().as_str() {
        "MEMORY" => {
            let key = args.next_arg()?;
            let key = ctx.open_key(&key);

            Ok(key
                .get_cbor_value()?
                .map(|v| v.mem_usage())
                .unwrap_or(0)
                .into())
        }
        "HELP" => {
            let results = vec![
                "MEMORY <key> [path] - reports memory usage",
                "HELP                - this message",
            ];
            Ok(results.into())
        }
        _ => Err(RedisError::Str(
            "ERR unknown subcommand - try `CBOR.DEBUG HELP`",
        )),
    }
}
