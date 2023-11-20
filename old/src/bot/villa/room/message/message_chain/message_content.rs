/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::message::message_object::message_content::image::Image;
use crate::api_type::message::message_object::message_content::mhy_post::MhyPost;
use crate::api_type::message::message_object::MessageObject;
use crate::bot::villa::room::message::message_chain::mhy_text::MhyText;

/// message content
#[derive(Debug, Clone, PartialEq)]
pub enum MessageContent {
  /// MHY:Text
  MhyText(MhyText),
  /// MHY:Image
  MhyImage(Image),
  /// MHY:Post
  MhyPost(MhyPost),
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

  /// try convert to [Image]
  pub fn as_mhy_image(&self) -> Option<&Image> {
    if let Self::MhyImage(image) = self {
      Some(image)
    } else {
      None
    }
  }

  /// try convert to [MhyPost]
  pub fn as_mhy_post(&self) -> Option<&MhyPost> {
    if let Self::MhyPost(mhy_post) = self {
      Some(mhy_post)
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
