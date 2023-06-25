/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use std::str::FromStr;

use crate::api_type::event::bot_event::bot_event_data::message_identifier::MessageIdentifier;
use crate::api_type::message::message_object::mentioned_info::MentionedInfo;
use crate::api_type::message::message_object::message_content::mhy_text::entity_data::EntityData;
use crate::api_type::message::message_object::message_content::mhy_text::text_entity::TextEntity;
use crate::api_type::message::message_object::message_content::mhy_text::MhyText as ApiMhyText;
use crate::api_type::message::message_object::message_content::MessageContent as ApiMessageContent;
use crate::api_type::message::message_object::MessageObject;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::link::Link;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::mention_bot::MentionBot;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::mention_user::MentionUser;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::villa_room_link::VillaRoomLink;
use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::MhyTextMsgComponent;
use crate::bot::villa::room::message::message_chain::message_content::MessageContent;
use crate::bot::villa::room::message::message_chain::mhy_text::MhyText;
use crate::error::{VError, VResult};
use crate::utils::unicode_utils::len_utf16;

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
      ApiMessageContent::MhyText(mhy_text) => parse_mhy_text(mhy_text)?,
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

fn parse_mhy_text(message: ApiMhyText) -> VResult<MessageContent> {
  let ApiMhyText { text, mut entities } = message;
  let len = len_utf16(&text);
  let mut result = Vec::with_capacity(entities.len().min(1));

  entities.sort_by_key(|it| it.offset);
  check_entities(&entities, len)?;

  let text_encoded = text.encode_utf16().collect::<Vec<_>>();
  let mut offset: usize = 0;

  for entity in entities {
    let previous = &text_encoded[offset..entity.offset as usize];
    if !previous.is_empty() {
      result.push(MhyTextMsgComponent::Text(String::from_utf16_lossy(
        previous,
      )))
    }

    let content = &text_encoded[entity.offset as usize..(entity.offset + entity.length) as usize];
    let content = String::from_utf16_lossy(content);

    fn parse_u64(string: impl AsRef<str>) -> VResult<u64> {
      Ok(u64::from_str(string.as_ref()).map_err(MessageChainParseError::from)?)
    }

    fn strip_prefix(string: impl Into<String>, prefix: impl AsRef<str>) -> String {
      let string = string.into();
      string
        .strip_prefix(prefix.as_ref())
        .map(|it| it.to_string())
        .unwrap_or_else(|| string)
    }

    let component = match entity.entity {
      EntityData::MentionedRobot { bot_id } => {
        MhyTextMsgComponent::MentionBot(MentionBot::new(strip_prefix(content, "@"), bot_id))
      }
      EntityData::MentionedUser { user_id } => MhyTextMsgComponent::MentionUser(MentionUser::new(
        strip_prefix(content, "@"),
        parse_u64(user_id)?,
      )),
      EntityData::MentionedAll => MhyTextMsgComponent::MentionAll,
      EntityData::VillaRoomLink { villa_id, room_id } => {
        MhyTextMsgComponent::VillaRoomLink(VillaRoomLink::new(
          strip_prefix(content, "#"),
          parse_u64(villa_id)?,
          parse_u64(room_id)?,
        ))
      }
      EntityData::Link { url } => MhyTextMsgComponent::Link(Link::new(content, url)),
    };

    result.push(component);

    offset = (entity.offset + entity.length) as usize;
  }

  if offset < len {
    let content = String::from_utf16_lossy(&text_encoded[offset..len]);
    result.push(MhyTextMsgComponent::Text(content));
  }

  Ok(MessageContent::MhyText(MhyText::new(result)))
}

/// need sorted entities
fn check_entities(entities: &[TextEntity], len: usize) -> Result<(), MessageChainParseError> {
  let mut offset = 0;

  for (idx, entity) in entities.iter().enumerate() {
    if entity.offset < offset {
      return Err(MessageChainParseError::EntitiesOverlap(
        entities[idx - 1].clone().into(),
        entity.clone().into(),
      ));
    }
    if entity.offset + entity.length > len as u64 {
      return Err(MessageChainParseError::EntitiesOutBoundary(
        len,
        entity.clone().into(),
      ));
    }

    offset = entity.offset + entity.length
  }

  Ok(())
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
  use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::link::Link;
  use crate::bot::villa::room::message::message_builder::mhy_text_msg_component::MhyTextMsgComponent;
  use crate::bot::villa::room::message::message_builder::MessageBuilder;
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
      .mhy_text()
      .mention_all()
      .text("123😶‍🌫️")
      .link_full("😶‍🌫️", "")
      .with_quote(&ident)
      .build();
    let msg = MessageChain::try_from(msg).unwrap();

    let expect = MessageChain::new(
      MessageContent::MhyText(MhyText::new(vec![
        MhyTextMsgComponent::MentionAll,
        MhyTextMsgComponent::Text(" 123😶‍🌫️ ".to_string()),
        MhyTextMsgComponent::Link(Link::new("😶‍🌫️".to_string(), "".to_string())),
      ])),
      Some(ident),
      Some(mentioned_info),
    );

    assert_eq!(msg, expect);
  }

  #[test]
  fn test_parse_empty() {
    let bot = default();
    let msg = bot.villa(0).room(0).message_builder().mhy_text().build();
    let msg = MessageChain::try_from(msg).unwrap();

    assert_eq!(msg, MessageChain::default());
  }
}
