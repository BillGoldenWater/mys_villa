/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::event::bot_event::bot_event_data::message_identifier::MessageIdentifier;
use crate::api_type::message::message_object::mentioned_info::MentionedInfo;
use crate::api_type::message::message_object::message_content::mhy_text::text_entity::TextEntity;
use crate::api_type::message::message_object::message_content::MessageContent as ApiMessageContent;
use crate::api_type::message::message_object::MessageObject;
use crate::bot::villa::room::message::message_chain::message_content::MessageContent;
use crate::error::VError;

/// message content
pub mod message_content;
/// content of MHY:Text
pub mod mhy_text;

/// message chain
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MessageChain {
  /// message content
  pub message_content: MessageContent,
  /// quote target
  pub quote: Option<MessageIdentifier>,
  /// members that need notify
  pub mentioned_info: Option<MentionedInfo>,
}

impl MessageChain {
  /// initialize with message content, quote and mentioned info
  pub fn new(
    message_content: MessageContent,
    quote: Option<MessageIdentifier>,
    mentioned_info: Option<MentionedInfo>,
  ) -> Self {
    Self {
      message_content,
      quote,
      mentioned_info,
    }
  }
}

impl TryFrom<MessageObject> for MessageChain {
  type Error = VError;

  fn try_from(value: MessageObject) -> Result<Self, Self::Error> {
    let message_content = match value.content {
      ApiMessageContent::MhyText(mhy_text) => MessageContent::MhyText(mhy_text.try_into()?),
      ApiMessageContent::Unknown(_) => MessageContent::Unknown(value.clone()),
    };

    Ok(MessageChain {
      message_content,
      quote: value
        .quote
        .map(|it| MessageIdentifier::new(it.quoted_message_id, it.quoted_message_send_time)),
      mentioned_info: value.mentioned_info,
    })
  }
}

/// parse error of message chain
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum MessageChainParseError {
  /// entities length exceeded content length
  #[error("entities out of boundary, content len: {0}, entity info: {1:?}")]
  EntitiesOutBoundary(usize, Box<TextEntity>),
  /// a entity is overlap with other entity
  #[error("entities overlapping, entity info: {0:?}, {1:?}")]
  EntitiesOverlap(Box<TextEntity>, Box<TextEntity>),
  /// failed to parse a number (user_uid)
  #[error("failed to parse number (user_uid) {0}")]
  NumberParse(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
  use crate::api_type::event::bot_event::bot_event_data::message_identifier::MessageIdentifier;
  use crate::api_type::message::message_object::mentioned_info::MentionedInfo;
  use crate::bot::default::default;
  use crate::bot::villa::room::message::message_builder::mhy_text_component::link::Link;
  use crate::bot::villa::room::message::message_builder::mhy_text_component::MhyTextMsgComponent;
  use crate::bot::villa::room::message::message_chain::message_content::MessageContent;
  use crate::bot::villa::room::message::message_chain::mhy_text::MhyText;
  use crate::bot::villa::room::message::message_chain::MessageChain;

  #[test]
  fn test_parse() {
    let bot = default();
    let ident = MessageIdentifier::new(String::new(), 0);
    let mentioned_info = MentionedInfo::All;

    let msg = bot
      .villa(0)
      .room(0)
      .message_builder()
      .with_quote(&ident)
      .mhy_text(|m| m.mention_all().text("123😶‍🌫️").link_full("😶‍🌫️", "", false))
      .build();

    let msg = MessageChain::try_from(msg).unwrap();

    let expect = MessageChain::new(
      MessageContent::MhyText(MhyText::new(vec![
        MhyTextMsgComponent::MentionAll,
        MhyTextMsgComponent::Text(" 123😶‍🌫️ ".to_string()),
        MhyTextMsgComponent::Link(Link::new("😶‍🌫️".to_string(), "".to_string(), false)),
      ])),
      Some(ident),
      Some(mentioned_info),
    );

    assert_eq!(msg, expect);
  }

  #[test]
  fn test_parse_empty() {
    let bot = default();
    let msg = bot
      .villa(0)
      .room(0)
      .message_builder()
      .mhy_text(|m| m)
      .build();
    let msg = MessageChain::try_from(msg).unwrap();

    assert_eq!(msg, MessageChain::default());
  }
}
