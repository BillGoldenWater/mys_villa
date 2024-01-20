/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api::villa_bot_api::villa_api::room_api::msg_content_info::msg_content::msg_mhy_text::mhy_text_entity::entity_data::EntityData;
use crate::bot::villa::room::message_object::message_content::mhy_text::span::entity::{
  entity_at::EntityAt, entity_link::EntityLink, entity_style::EntityStyle,
};
use crate::bot::villa::room::message_object::message_content::mhy_text::span::entity::entity_link::link_extern::LinkExtern;
use crate::bot::villa::room::message_object::message_content::mhy_text::span::entity::entity_link::link_room::LinkRoom;
use crate::utils::fp_utils::FpUtils;

pub mod entity_at;
pub mod entity_link;
pub mod entity_style;
pub mod error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Entity {
  At(EntityAt),
  Link(EntityLink),
  Style(EntityStyle),
}

impl From<Entity> for EntityData {
  fn from(value: Entity) -> Self {
    match value {
      Entity::At(at) => match at {
        EntityAt::All => EntityData::MentionAll,
        EntityAt::User(uid) => EntityData::MentionedUser {
          user_id: uid.to_string(),
        },
        EntityAt::Bot(bot_id) => EntityData::MentionedRobot { bot_id },
      },
      Entity::Link(link) => match link {
        EntityLink::Room(link) => EntityData::VillaRoomLink {
          villa_id: link.villa_id.to_string(),
          room_id: link.room_id.to_string(),
        },
        EntityLink::Extern(link) => EntityData::Link {
          url: link.url,
          requires_bot_access_token: link.require_access_token,
        },
      },
      Entity::Style(style) => EntityData::Style {
        font_style: style.into(),
      },
    }
  }
}

impl TryFrom<EntityData> for Entity {
  type Error = error::Error;

  fn try_from(value: EntityData) -> Result<Self, Self::Error> {
    match value {
      EntityData::MentionAll => Entity::At(EntityAt::All),
      EntityData::MentionedUser { user_id } => Entity::At(EntityAt::User(user_id.parse()?)),
      EntityData::MentionedRobot { bot_id } => Entity::At(EntityAt::Bot(bot_id)),
      EntityData::VillaRoomLink { villa_id, room_id } => Entity::Link(EntityLink::Room(
        LinkRoom::new(villa_id.parse()?, room_id.parse()?),
      )),
      EntityData::Link {
        url,
        requires_bot_access_token,
      } => Entity::Link(EntityLink::Extern(LinkExtern::new(
        url,
        requires_bot_access_token,
      ))),
      EntityData::Style { font_style } => Entity::Style(EntityStyle::from(font_style)),
    }
    .ok()
  }
}
