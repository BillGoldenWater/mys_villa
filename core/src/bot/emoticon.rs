/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api::villa_bot_api::get_all_emoticons::emoticon::Emoticon as RawEmoticon;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Emoticon {
  pub id: u64,
  pub text: String,

  pub icon_url: String,
}

impl From<RawEmoticon> for Emoticon {
  fn from(value: RawEmoticon) -> Self {
    Self {
      id: value.emoticon_id,
      text: value.describe_text,

      icon_url: value.icon,
    }
  }
}
