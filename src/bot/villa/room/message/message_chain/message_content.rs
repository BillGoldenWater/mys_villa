/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::message::message_object::message_content::image::Image;
use crate::api_type::message::message_object::MessageObject;
use crate::bot::villa::room::message::message_chain::mhy_text::MhyText;

/// message content
#[derive(Debug, Clone, PartialEq)]
pub enum MessageContent {
  /// MHY:Text
  MhyText(MhyText),
  /// MHY:Image
  MhyImage(Image),
  /// unknown message
  Unknown(MessageObject),
}

impl MessageContent {
  /// try convert to [MhyText]
  pub fn as_mhy_text(&self) -> Option<&MhyText> {
    if let Self::MhyText(mhy_text) = self {
      Some(mhy_text)
    } else {
      None
    }
  }

  /// try convert to unknown
  pub fn as_unknown(&self) -> Option<&MessageObject> {
    if let Self::Unknown(message_object) = self {
      Some(message_object)
    } else {
      None
    }
  }
}

impl Default for MessageContent {
  fn default() -> Self {
    Self::MhyText(MhyText::default())
  }
}
