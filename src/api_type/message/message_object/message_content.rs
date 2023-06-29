/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::message::message_object::message_content::image::Image;
use crate::api_type::message::message_object::message_content::mhy_text::MhyText;
use crate::api_type::message::message_object::message_content::unknown::Unknown;
use serde::{Deserialize, Serialize};

/// definition of image
pub mod image;
/// definition of mhy text
pub mod mhy_text;
/// definition of unknown
pub mod unknown;

/// message content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
  /// MHY:Text
  MhyText(MhyText),
  /// MHY:Image
  MhyImage(Image),
  /// unknown message
  Unknown(Unknown),
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
  pub fn as_unknown(&self) -> Option<&Unknown> {
    if let Self::Unknown(unknown) = self {
      Some(unknown)
    } else {
      None
    }
  }
}
