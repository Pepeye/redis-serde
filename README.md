# R e d i s   S e r d e

Learning and credit to following resources

```rust
// NOTE:
// solutions
// https://github.com/mitsuhiko/redis-rs/issues/352
// https://users.rust-lang.org/t/how-impl-a-trait-of-fromredisvalue-for-more-structs/67532
// https://github.com/mitsuhiko/redis-rs/issues/128
//
impl FromRedisValue for User {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        convert(v)
    }
}

fn convert<'a, T>(v: &'a Value) -> RedisResult<T>
where T:  Deserialize<'a>,
{
    match *v {
        redis::Value::Data(ref bytes) => {
            match serde_json::from_slice::<T>(bytes) {
                Err(_) => Err(((redis::ErrorKind::TypeError, "JSON deserialize failed.")).into()),
                Ok(v) => Ok(v),
            }
        },
        _ => Err(((ErrorKind::ResponseError, "Invalid response type, not JSON compatible.")).into()),
    }
}


impl ToRedisArgs for User {
    fn write_redis_args<W>(&self, out: &mut W)
    where
      W: ?Sized + redis::RedisWrite,
    {
      out.write_arg(&serde_json::to_vec(self).expect("JSON encoding failed"));
    }
}
```

Finally credit to [Bajix](https://github.com/Bajix) and [Clia](https://github.com/clia) for work here .[derive-redis-json](https://github.com/Bajix/derive-redis-json) which is the culmination of above research and learning

