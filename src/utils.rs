use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};

/// deserialize from string and serialize to string
pub mod serde_obj_str;
/// for unicode related utilities
pub mod unicode_utils;

/// deserialize a string vec into number vec
pub fn deserialize_number_vec_from_string_vec<'de, T, D>(
  deserializer: D,
) -> Result<Vec<T>, D::Error>
where
  D: Deserializer<'de>,
  T: FromStr + serde::Deserialize<'de>,
  <T as FromStr>::Err: Display,
{
  #[derive(Deserialize)]
  #[serde(untagged)]
  enum StringOrInt<T> {
    String(String),
    Number(T),
  }

  Vec::<StringOrInt<T>>::deserialize(deserializer)?
    .into_iter()
    .map(|it| match it {
      StringOrInt::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
      StringOrInt::Number(i) => Ok(i),
    })
    .collect()
}
