use crate::util::{apply_changes, CborExt, CborKeyWritable, CborPathExt, NextArgExt};
use cbor_data::{Cbor, CborBuilder, CborOwned, ItemKind, Writer};
use cborpath::{builder::IntoCborOwned, CborPath};
use redis_module::{Context, RedisError, RedisResult, RedisString, RedisValue};
use std::borrow::Cow;

///
/// CBOR.NUMINCRBY key path value
///
pub fn cbor_num_incr_by(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.iter().skip(1);

    let key_name = args.next_arg()?;
    let path = args.next_arg()?;
    let value = args.next_arg()?;

    let key = ctx.open_key_writable(key_name);
    let cbor_path = CborPath::from_arg(path)?;
    let value = Cbor::from_arg(value)?;

    let Some(existing) = key.get_cbor_value()? else {
        return Err(RedisError::nonexistent_key());
    };

    let (new_value, new_nums) = num_incr_by(existing, &cbor_path, value);

    if let Some(new_value) = new_value {
        key.set_cbor_value(new_value)?;
        apply_changes(ctx, "cbor.numincrby", key_name)?;
    }

    Ok(new_nums.into())
}

fn num_incr_by(
    existing: &CborOwned,
    cbor_path: &CborPath,
    value: &Cbor,
) -> (Option<CborOwned>, Vec<RedisValue>) {
    let mut new_nums = Vec::<RedisValue>::new();

    let new_value = cbor_path
        .write(existing, |old_value| {
            // `Neg` variant carries `-1 - x`
            match (old_value.kind(), value.kind()) {
                (ItemKind::Pos(v1), ItemKind::Pos(v2)) => {
                    let result = IntoCborOwned::into(v1 + v2);
                    new_nums.push(RedisValue::StringBuffer(result.clone().into_vec()));
                    Ok(Some(Cow::Owned(result)))
                }
                (ItemKind::Pos(v1), ItemKind::Neg(v2)) => {
                    let result = IntoCborOwned::into(v1 as i64 - 1 - (v2 as i64));
                    new_nums.push(RedisValue::StringBuffer(result.clone().into_vec()));
                    Ok(Some(Cow::Owned(result)))
                }
                (ItemKind::Neg(v1), ItemKind::Pos(v2)) => {
                    let result = IntoCborOwned::into(v2 as i64 - 1 - (v1 as i64));
                    new_nums.push(RedisValue::StringBuffer(result.clone().into_vec()));
                    Ok(Some(Cow::Owned(result)))
                }
                (ItemKind::Neg(v1), ItemKind::Neg(v2)) => {
                    let result = CborBuilder::new().write_neg(v1 + v2 + 1, None);
                    new_nums.push(RedisValue::StringBuffer(result.clone().into_vec()));
                    Ok(Some(Cow::Owned(result)))
                }
                (ItemKind::Float(v1), ItemKind::Float(v2)) => {
                    let result = IntoCborOwned::into(v1 + v2);
                    new_nums.push(RedisValue::StringBuffer(result.clone().into_vec()));
                    Ok(Some(Cow::Owned(result)))
                }
                _ => {
                    new_nums.push(RedisValue::Null);
                    Ok(Some(Cow::Borrowed(old_value)))
                }
            }
        })
        .unwrap();

    (new_value, new_nums)
}

#[cfg(test)]
mod tests {
    use super::num_incr_by;
    use crate::util::{cbor_to_diag, diag_to_bytes, diag_to_cbor};
    use cborpath::{builder::segment, CborPath};
    use redis_module::RedisValue;

    #[test]
    fn incr_by_integer() {
        let cbor = diag_to_cbor("[2,-2]");
        let cbor_path = CborPath::builder().wildcard().build();

        let value = diag_to_cbor("1");
        let (new_value, nem_nums) = num_incr_by(&cbor, &cbor_path, &value);
        assert_eq!("[3,-1]", cbor_to_diag(&new_value.unwrap()));
        assert_eq!(
            vec![
                RedisValue::StringBuffer(diag_to_bytes("3")),
                RedisValue::StringBuffer(diag_to_bytes("-1")),
            ],
            nem_nums
        );

        let value = diag_to_cbor("-1");
        let (new_value, nem_nums) = num_incr_by(&cbor, &cbor_path, &value);
        assert_eq!("[1,-3]", cbor_to_diag(&new_value.unwrap()));
        assert_eq!(
            vec![
                RedisValue::StringBuffer(diag_to_bytes("1")),
                RedisValue::StringBuffer(diag_to_bytes("-3")),
            ],
            nem_nums
        );
    }

    #[test]
    fn incr_by_float() {
        let cbor = diag_to_cbor("[12.13,-12.12]");
        let cbor_path = CborPath::builder().wildcard().build();

        let value = diag_to_cbor("2.02");
        let (new_value, nem_nums) = num_incr_by(&cbor, &cbor_path, &value);
        assert_eq!(diag_to_cbor("[14.15,-10.1]"), new_value.unwrap());
        assert_eq!(
            vec![
                RedisValue::StringBuffer(diag_to_bytes("14.15")),
                RedisValue::StringBuffer(diag_to_bytes("-10.1")),
            ],
            nem_nums
        );
    }

    #[test]
    fn incr_by() {
        let cbor = diag_to_cbor(r#"{"a":"b","b":[{"a":2}, {"a":5}, {"a":"c"}]}"#);
        let cbor_path = CborPath::builder().descendant(segment().key("a")).build();
        let value = diag_to_cbor("2");

        let (new_value, nem_nums) = num_incr_by(&cbor, &cbor_path, &value);
        assert_eq!(
            r#"{"a":"b","b":[{"a":4},{"a":7},{"a":"c"}]}"#,
            cbor_to_diag(&new_value.unwrap())
        );
        assert_eq!(
            vec![
                RedisValue::Null,
                RedisValue::StringBuffer(diag_to_bytes("4")),
                RedisValue::StringBuffer(diag_to_bytes("7")),
                RedisValue::Null
            ],
            nem_nums
        );
    }
}
