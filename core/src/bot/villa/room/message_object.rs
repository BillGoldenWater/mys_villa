/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
  api::villa_bot_api::villa_api::room_api::msg_content_info::MsgContentInfo,
  bot::villa::room::message_object::message_content::MessageContent, utils::fp_utils::FpUtils,
};

pub mod error;
pub mod message_content;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MessageObject {
  pub content: MessageContent,
}

impl MessageObject {
  pub fn new(content: MessageContent) -> Self {
    Self { content }
  }
}

impl From<MessageObject> for MsgContentInfo {
  fn from(value: MessageObject) -> Self {
    Self {
      content: value.content.into(),
    }
  }
}

impl TryFrom<MsgContentInfo> for MessageObject {
  type Error = error::Error;

  fn try_from(value: MsgContentInfo) -> Result<Self, Self::Error> {
    Self {
      content: value.content.try_into()?,
    }
    .ok()
  }
}
