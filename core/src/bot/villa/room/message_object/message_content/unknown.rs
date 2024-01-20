/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde_json::Value;

use crate::api::villa_bot_api::villa_api::room_api::msg_content_info::msg_content::msg_unknown::MsgUnknown;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Unknown {
  pub object_name: Value,
  pub content: Value,
}

impl Unknown {
  pub fn new(object_name: Value, content: Value) -> Self {
    Self {
      object_name,
      content,
    }
  }
}

impl From<Unknown> for MsgUnknown {
  fn from(value: Unknown) -> Self {
    Self::new(value.object_name, value.content)
  }
}

impl From<MsgUnknown> for Unknown {
  fn from(value: MsgUnknown) -> Self {
    Self::new(value.object_name, value.content)
  }
}
