/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::bot::villa::room::message::message_builder::mhy_text_component::link::Link;
use crate::bot::villa::room::message::message_builder::mhy_text_component::mention_bot::MentionBot;
use crate::bot::villa::room::message::message_builder::mhy_text_component::mention_user::MentionUser;
use crate::bot::villa::room::message::message_builder::mhy_text_component::villa_room_link::VillaRoomLink;

/// link content
pub mod link;
/// mention bot content
pub mod mention_bot;
/// mention user content
pub mod mention_user;
/// villa room link content
pub mod villa_room_link;

/// message component of mhy text
#[derive(Debug, Clone, PartialEq)]
pub enum MhyTextMsgComponent {
  /// text component
  Text(String),
  /// spacer component
  Spacer(String),
  /// mention user
  MentionUser(MentionUser),
  /// mention all
  MentionAll,
  /// mention bot
  MentionBot(MentionBot),
  /// villa room link
  VillaRoomLink(VillaRoomLink),
  /// link
  Link(Link),
}

impl MhyTextMsgComponent {
  /// try convert to text
  pub fn as_text(&self) -> Option<&str> {
    if let Self::Text(text) = self {
      Some(text.as_str())
    } else {
      None
    }
  }

  /// try convert to spacer
  pub fn as_spacer(&self) -> Option<&str> {
    if let Self::Spacer(spacer) = self {
      Some(spacer.as_str())
    } else {
      None
    }
  }

  /// try convert to mention user
  pub fn as_mention_user(&self) -> Option<&MentionUser> {
    if let Self::MentionUser(mention) = self {
      Some(mention)
    } else {
      None
    }
  }

  /// try convert to mention bot
  pub fn as_mention_bot(&self) -> Option<&MentionBot> {
    if let Self::MentionBot(mention) = self {
      Some(mention)
    } else {
      None
    }
  }

  /// try convert to villa room link
  pub fn as_villa_room_link(&self) -> Option<&VillaRoomLink> {
    if let Self::VillaRoomLink(room_link) = self {
      Some(room_link)
    } else {
      None
    }
  }

  /// try convert to link
  pub fn as_link(&self) -> Option<&Link> {
    if let Self::Link(text_link) = self {
      Some(text_link)
    } else {
      None
    }
  }

  /// check this instance is or isn't mention all
  pub fn is_mention_all(&self) -> bool {
    matches!(self, Self::MentionAll)
  }
}
