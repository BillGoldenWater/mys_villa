use serde::de::{DeserializeOwned, Error as DeError};
use serde::ser::Error as SerError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// deserialize string into object
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
  D: Deserializer<'de>,
  T: DeserializeOwned,
{
  let string = String::deserialize(deserializer)?;
  serde_json::from_str(&string)
    .map_err(|it| D::Error::custom(format!("failed to deserialize from string: {it:?}")))
}

/// serialize object into string
pub fn serialize<T, D>(value: &T, serializer: D) -> Result<D::Ok, D::Error>
where
  D: Serializer,
  T: Serialize,
{
  let string = serde_json::to_string(value)
    .map_err(|it| D::Error::custom(format!("failed to serialize to string: {it:?}")))?;
  string.serialize(serializer)
}
