/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::de::Error as DeError;
use serde::ser::Error as SerError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::api_type::message::message_mhy_text::MessageMhyText;

/// message object
#[derive(Debug, Clone, PartialEq)]
pub enum MessageObject {
  /// type MHY:Text
  MhyText(MessageMhyText),
  /// unknown type
  Unknown {
    /// object name
    object_name: i32,
    /// content
    content: String,
  },
}

impl<'de> Deserialize<'de> for MessageObject {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    #[derive(Debug, Deserialize)]
    struct MessageObjectRaw {
      object_name: i32,
      content: String,
    }

    let raw = MessageObjectRaw::deserialize(deserializer)?;
    Ok(match raw.object_name {
      1 => MessageObject::MhyText(
        serde_json::from_str(&raw.content)
          .map_err(|err| D::Error::custom(format!("failed to deserialize from string: {err:?}")))?,
      ),
      object_name => MessageObject::Unknown {
        object_name,
        content: raw.content,
      },
    })
  }
}

impl Serialize for MessageObject {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    #[derive(Debug, Serialize)]
    struct MessageObjectRaw {
      object_name: String,
      msg_content: String,
    }

    match self {
      MessageObject::MhyText(content) => MessageObjectRaw {
        object_name: "MHY:Text".to_string(),
        msg_content: serde_json::to_string(content)
          .map_err(|err| S::Error::custom(format!("failed to serialize to string: {err:?}")))?,
      },
      MessageObject::Unknown {
        object_name,
        content,
      } => MessageObjectRaw {
        object_name: object_name.to_string(),
        msg_content: content.clone(),
      },
    }
    .serialize(serializer)
  }
}
