/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
  api::villa_bot_api::villa_api::room_api::msg_content_info::msg_content::MsgContent,
  bot::villa::room::message_object::message_content::{mhy_text::MhyText, unknown::Unknown},
  utils::fp_utils::FpUtils,
};

pub mod error;
pub mod mhy_text;
pub mod unknown;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MessageContent {
  MhyText(MhyText),
  Unknown(Unknown),
}

impl From<MessageContent> for MsgContent {
  fn from(value: MessageContent) -> Self {
    match value {
      MessageContent::MhyText(mhy_text) => Self::MhyText(mhy_text.into()),
      MessageContent::Unknown(unknown) => Self::Unknown(unknown.into()),
    }
  }
}

impl TryFrom<MsgContent> for MessageContent {
  type Error = error::Error;

  fn try_from(value: MsgContent) -> Result<Self, Self::Error> {
    match value {
      MsgContent::MhyText(mhy_text) => Self::MhyText(mhy_text.try_into()?),
      MsgContent::Unknown(unknown) => Self::Unknown(unknown.into()),
    }
    .ok()
  }
}
