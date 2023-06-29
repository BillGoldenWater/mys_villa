/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::message::message_object::message_content::mhy_text::entity_data::EntityData;
use crate::api_type::message::message_object::message_content::mhy_text::text_entity::TextEntity;
use crate::api_type::message::message_object::message_content::mhy_text::MhyText as ApiMhyText;
use crate::bot::villa::room::message::message_builder::mhy_text_component::link::Link;
use crate::bot::villa::room::message::message_builder::mhy_text_component::mention_bot::MentionBot;
use crate::bot::villa::room::message::message_builder::mhy_text_component::mention_user::MentionUser;
use crate::bot::villa::room::message::message_builder::mhy_text_component::villa_room_link::VillaRoomLink;
use crate::bot::villa::room::message::message_builder::mhy_text_component::MhyTextMsgComponent;
use crate::bot::villa::room::message::message_chain::MessageChainParseError;
use crate::error::{VError, VResult};
use crate::utils::unicode_utils::len_utf16;
use std::str::FromStr;

/// mhy text
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MhyText {
  /// message content
  pub content: Vec<MhyTextMsgComponent>,
}

impl MhyText {
  /// initialize with content, quote and mentioned_info
  pub fn new(content: Vec<MhyTextMsgComponent>) -> Self {
    Self { content }
  }
}

impl TryFrom<ApiMhyText> for MhyText {
  type Error = VError;

  fn try_from(value: ApiMhyText) -> Result<Self, Self::Error> {
    let ApiMhyText { text, mut entities } = value;
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
        EntityData::MentionedUser { user_id } => MhyTextMsgComponent::MentionUser(
          MentionUser::new(strip_prefix(content, "@"), parse_u64(user_id)?),
        ),
        EntityData::MentionedAll => MhyTextMsgComponent::MentionAll,
        EntityData::VillaRoomLink { villa_id, room_id } => {
          MhyTextMsgComponent::VillaRoomLink(VillaRoomLink::new(
            strip_prefix(content, "#"),
            parse_u64(villa_id)?,
            parse_u64(room_id)?,
          ))
        }
        EntityData::Link {
          url,
          requires_bot_access_token,
        } => MhyTextMsgComponent::Link(Link::new(content, url, requires_bot_access_token)),
      };

      result.push(component);

      offset = (entity.offset + entity.length) as usize;
    }

    if offset < len {
      let content = String::from_utf16_lossy(&text_encoded[offset..len]);
      result.push(MhyTextMsgComponent::Text(content));
    }

    Ok(MhyText::new(result))
  }
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
