use crate::redis_cbor_type::REDIS_CBOR_TYPE;
#[cfg(test)]
use cbor_data::Cbor;
use cbor_data::CborOwned;
#[cfg(test)]
use cbor_diag::{parse_bytes, parse_diag};
use redis_module::{
    key::{RedisKey, RedisKeyWritable},
    Context, NotifyEvent, RedisError, RedisString, Status,
};

pub fn apply_changes(
    ctx: &Context,
    command: &str,
    key_name: &RedisString,
) -> Result<(), RedisError> {
    if ctx.notify_keyspace_event(NotifyEvent::MODULE, command, key_name) != Status::Ok {
        Err(RedisError::Str("failed notify key space event"))
    } else {
        ctx.replicate_verbatim();
        Ok(())
    }
}

pub trait CborKey {
    fn get_cbor_value(&self) -> Result<Option<&CborOwned>, RedisError>;
}

pub trait CborKeyWritable {
    fn get_cbor_value(&self) -> Result<Option<&CborOwned>, RedisError>;
    fn set_cbor_value(&self, value: CborOwned) -> Result<(), RedisError>;
}

impl CborKey for RedisKey {
    #[inline]
    fn get_cbor_value(&self) -> Result<Option<&CborOwned>, RedisError> {
        self.get_value::<CborOwned>(&REDIS_CBOR_TYPE)
    }
}

impl CborKeyWritable for RedisKeyWritable {
    #[inline]
    fn get_cbor_value(&self) -> Result<Option<&CborOwned>, RedisError> {
        match self.get_value::<CborOwned>(&REDIS_CBOR_TYPE) {
            Ok(Some(result)) => Ok(Some(result)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    #[inline]
    fn set_cbor_value(&self, value: CborOwned) -> Result<(), RedisError> {
        self.set_value(&REDIS_CBOR_TYPE, value)
    }
}

#[cfg(test)]
pub fn diag_to_cbor(cbor_diag_str: &str) -> CborOwned {
    let buf = diag_to_bytes(cbor_diag_str);
    CborOwned::canonical(&buf).unwrap()
}

#[cfg(test)]
pub fn diag_to_bytes(cbor_diag_str: &str) -> Vec<u8> {
    parse_diag(cbor_diag_str).unwrap().to_bytes()
}

#[cfg(test)]
pub fn cbor_to_diag(cbor: &Cbor) -> String {
    bytes_to_diag(cbor.as_ref())
}

#[cfg(test)]
pub fn bytes_to_diag(cbor: &[u8]) -> String {
    parse_bytes(cbor).unwrap().to_diag()
}
