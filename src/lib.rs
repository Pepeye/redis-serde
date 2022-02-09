#![recursion_limit = "128"]

use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// .
///
/// # Examples
///
/// ```
/// use redis_serde::redis_json;
///
/// assert_eq!(redis_json(input), );
/// ```
#[proc_macro_derive(RedisJson)]
pub fn redis_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_type = input.ident;

    let expanded = quote! {
        impl redis::ToRedisArgs for #struct_type {
            fn write_redis_args<W>(&self, out: &mut W)
            where
              W: ?Sized + redis::RedisWrite,
            {
              out.write_arg(&serde_json::to_vec(self).expect("JSON encoding failed"));
            }
        }

        impl redis::FromRedisValue for #struct_type {
            fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
              match *v {
                redis::Value::Data(ref bytes) => Ok(serde_json::from_slice(bytes).map_err(|err| {
                  (
                    redis::ErrorKind::TypeError,
                    "JSON deserialize failed",
                    err.to_string(),
                  )
                })?),
                _ => Err(
                  (
                    redis::ErrorKind::TypeError,
                    "invalid response type for JSON",
                  )
                    .into(),
                ),
              }
            }
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
