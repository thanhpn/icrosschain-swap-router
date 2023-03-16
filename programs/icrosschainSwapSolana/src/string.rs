//! Fixed length string types
use {
    arraystring::{typenum::U64, ArrayString},
    serde::Serialize,
};

/// Fixed size array to store UTF-8 strings on blockchain.
pub type ArrayString64 = ArrayString<U64>;

pub fn to_pretty_json<T>(object: &T) -> Result<String, serde_json::Error>
where
    T: ?Sized + Serialize,
{
    serde_json::to_string_pretty(&object)
}
