/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::utils::deserialize_number_vec_from_string_vec;

/// room allow send message range
#[derive(Debug, Eq, PartialEq)]
pub enum RoomAllowSendRange {
  /// all member can send
  All,
  /// only member has one of these role can send
  Role(Vec<u64>),
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct SendMsgAuthRange {
  is_all_send_msg: bool,
  #[serde(deserialize_with = "deserialize_number_vec_from_string_vec")]
  roles: Vec<u64>,
}

impl Serialize for RoomAllowSendRange {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let send_msg_auth_range = match self {
      RoomAllowSendRange::All => SendMsgAuthRange {
        is_all_send_msg: true,
        ..Default::default()
      },
      RoomAllowSendRange::Role(roles) => SendMsgAuthRange {
        roles: roles.clone(),
        ..Default::default()
      },
    };

    send_msg_auth_range.serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for RoomAllowSendRange {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let send_msg_auth_range = SendMsgAuthRange::deserialize(deserializer)?;

    let result = if send_msg_auth_range.is_all_send_msg {
      Self::All
    } else {
      Self::Role(send_msg_auth_range.roles)
    };

    Ok(result)
  }
}

#[cfg(test)]
#[doc(hidden)]
mod tests {
  use crate::api_type::room::room_allow_send_range::RoomAllowSendRange;

  #[test]
  fn test_serialize() {
    let value = RoomAllowSendRange::All;
    assert_eq!(
      serde_json::to_string(&value).unwrap(),
      r#"{"is_all_send_msg":true,"roles":[]}"#
    );

    let value = RoomAllowSendRange::Role(vec![]);
    assert_eq!(
      serde_json::to_string(&value).unwrap(),
      r#"{"is_all_send_msg":false,"roles":[]}"#
    );

    let value = RoomAllowSendRange::Role(vec![1, 2, 3]);
    assert_eq!(
      serde_json::to_string(&value).unwrap(),
      r#"{"is_all_send_msg":false,"roles":[1,2,3]}"#
    );
  }

  #[test]
  fn test_deserialize() {
    let value = r#"{"is_all_send_msg":true,"roles":[]}"#;
    assert_eq!(
      serde_json::from_str::<RoomAllowSendRange>(value).unwrap(),
      RoomAllowSendRange::All
    );

    let value = r#"{"is_all_send_msg":false,"roles":[]}"#;
    assert_eq!(
      serde_json::from_str::<RoomAllowSendRange>(value).unwrap(),
      RoomAllowSendRange::Role(vec![])
    );

    let value = r#"{"is_all_send_msg":false,"roles":[1,2,3]}"#;
    assert_eq!(
      serde_json::from_str::<RoomAllowSendRange>(value).unwrap(),
      RoomAllowSendRange::Role(vec![1, 2, 3])
    );

    let value = r#"{"is_all_send_msg":false,"roles":["1","2","3"]}"#;
    assert_eq!(
      serde_json::from_str::<RoomAllowSendRange>(value).unwrap(),
      RoomAllowSendRange::Role(vec![1, 2, 3])
    );
  }
}
