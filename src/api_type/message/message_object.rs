/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::message::message_object::mentioned_info::MentionedInfo;
use crate::api_type::message::message_object::message_content::unknown::Unknown;
use crate::api_type::message::message_object::message_content::MessageContent;
use crate::api_type::message::message_object::quote_info::QuoteInfo;
use serde::de::{DeserializeOwned, Error as DeError};
use serde::ser::Error as SerError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

/// definition of mentioned info
pub mod mentioned_info;
/// definition of message content
pub mod message_content;
/// definition of quote info
pub mod quote_info;

/// message object
#[derive(Debug, Clone, PartialEq)]
pub struct MessageObject {
  /// message content
  pub content: MessageContent,
  /// mentioned info
  pub mentioned_info: Option<MentionedInfo>,
  /// quote
  pub quote: Option<QuoteInfo>,
}

impl MessageObject {
  /// initialize with content, mentioned info and quote
  pub fn new(
    content: MessageContent,
    mentioned_info: Option<MentionedInfo>,
    quote: Option<QuoteInfo>,
  ) -> Self {
    Self {
      content,
      mentioned_info,
      quote,
    }
  }
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

    #[derive(Debug, Deserialize)]
    struct MsgContentInfo {
      content: Value,
      #[serde(default)]
      mentioned_info: Option<MentionedInfo>,
      #[serde(default)]
      quote: Option<QuoteInfo>,
    }

    impl MsgContentInfo {
      fn convert_to_object<
        'a,
        Content: DeserializeOwned,
        F: FnOnce(Content) -> MessageContent,
        De: Deserializer<'a>,
      >(
        self,
        wrapper: F,
      ) -> Result<MessageObject, De::Error> {
        let MsgContentInfo {
          content,
          mentioned_info,
          quote,
        } = self;
        Ok(MessageObject {
          content: wrapper(
            serde_json::from_value::<Content>(content).map_err(|err| {
              De::Error::custom(format!("failed to deserialize content: {err:?}"))
            })?,
          ),
          mentioned_info,
          quote,
        })
      }

      fn convert_to_unknown_object(self, object_name: i32) -> MessageObject {
        let MsgContentInfo {
          content,
          mentioned_info,
          quote,
        } = self;
        MessageObject {
          content: MessageContent::Unknown(Unknown::new(object_name.to_string(), content)),
          mentioned_info,
          quote,
        }
      }
    }

    let raw = MessageObjectRaw::deserialize(deserializer)?;

    fn deserialize_content<'a, De: Deserializer<'a>>(
      content: impl AsRef<str>,
    ) -> Result<MsgContentInfo, De::Error> {
      serde_json::from_str(content.as_ref())
        .map_err(|err| De::Error::custom(format!("failed to deserialize from string: {err:?}")))
    }

    Ok(match raw.object_name {
      1 => deserialize_content::<D>(raw.content)?
        .convert_to_object::<_, _, D>(MessageContent::MhyText)?,
      object_name => deserialize_content::<D>(raw.content)?.convert_to_unknown_object(object_name),
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

    #[derive(Debug, Serialize)]
    #[serde(untagged)]
    enum MessageContentRaw<'a> {
      Normal(&'a MessageContent),
      Unknown(&'a Value),
    }

    #[derive(Debug, Serialize)]
    struct MsgContentInfo<'a> {
      content: MessageContentRaw<'a>,
      mentioned_info: &'a Option<MentionedInfo>,
      quote: &'a Option<QuoteInfo>,
    }

    let object_name = match &self.content {
      MessageContent::MhyText(_) => "MHY:Text",
      MessageContent::Unknown(unknown) => unknown.object_name.as_str(),
    }
    .to_string();

    let content = match &self.content {
      MessageContent::Unknown(unknown) => MessageContentRaw::Unknown(&unknown.content),
      other => MessageContentRaw::Normal(other),
    };

    let content = MsgContentInfo {
      content,
      mentioned_info: &self.mentioned_info,
      quote: &self.quote,
    };

    MessageObjectRaw {
      object_name,
      msg_content: serde_json::to_string(&content)
        .map_err(|err| S::Error::custom(format!("failed to serialize to string: {err:?}")))?,
    }
    .serialize(serializer)
  }
}
