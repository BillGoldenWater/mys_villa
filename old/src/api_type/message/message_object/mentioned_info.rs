/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

/// mentioned info
#[derive(Debug, Clone, PartialEq)]
pub enum MentionedInfo {
  /// all
  All,
  /// partial (user_id)
  Partial(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MentionedInfoRaw {
  r#type: u32,
  user_id_list: Vec<String>,
}

impl Serialize for MentionedInfo {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      MentionedInfo::All => MentionedInfoRaw {
        r#type: 1,
        user_id_list: vec![],
      },
      MentionedInfo::Partial(members) => MentionedInfoRaw {
        r#type: 2,
        user_id_list: members.clone(),
      },
    }
    .serialize(serializer)
  }
}

impl<'de> Deserialize<'de> for MentionedInfo {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    let raw = MentionedInfoRaw::deserialize(deserializer)?;

    if raw.r#type == 1 {
      Ok(MentionedInfo::All)
    } else if raw.r#type == 2 {
      Ok(MentionedInfo::Partial(raw.user_id_list))
    } else {
      Err(Error::custom(format!(
        "failed to parse mentioned info, unknown type {}",
        raw.r#type
      )))
    }
  }
}
