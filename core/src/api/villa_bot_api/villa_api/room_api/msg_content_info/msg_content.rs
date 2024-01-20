/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde_json::Value;

use crate::api::villa_bot_api::villa_api::room_api::msg_content_info::msg_content::{
  msg_mhy_text::MsgMhyText, msg_unknown::MsgUnknown,
};

pub mod msg_mhy_text;
pub mod msg_unknown;

#[derive(Debug, Clone, PartialEq)]
pub enum MsgContent {
  MhyText(MsgMhyText),
  Unknown(MsgUnknown),
}

impl MsgContent {
  pub fn as_str_name(&self) -> &str {
    match self {
      MsgContent::MhyText(_) => "MHY:Text",
      MsgContent::Unknown(MsgUnknown { object_name, .. }) => match object_name {
        Value::String(str) => str.as_str(),
        _ => "",
      },
    }
  }
}
