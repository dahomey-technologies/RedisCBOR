use redis_module::{Context, NotifyEvent, RedisError, RedisString, Status};

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
