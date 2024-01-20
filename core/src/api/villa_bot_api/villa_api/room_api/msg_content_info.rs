/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
  api::{
    api_error::ApiResult,
    villa_bot_api::villa_api::room_api::msg_content_info::msg_content::{
      msg_unknown::MsgUnknown, MsgContent,
    },
  },
  utils::fp_utils::FpUtils,
};

pub mod msg_content;

#[derive(Debug, Clone, PartialEq)]
pub struct MsgContentInfo {
  pub content: MsgContent,
}

impl MsgContentInfo {
  pub fn try_from_raw_with_u64_name(
    value: MsgContentInfoForSendAndRecv,
    object_name: u64,
  ) -> ApiResult<Self> {
    match object_name {
      1 => Self {
        content: MsgContent::MhyText(serde_json::from_value(value.content)?),
      },
      unknown => Self {
        content: MsgContent::Unknown(MsgUnknown::new(
          Value::Number(serde_json::Number::from(unknown)),
          value.content,
        )),
      },
    }
    .ok()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsgContentInfoForSendAndRecv {
  pub content: Value,
}

impl From<MsgContentInfo> for MsgContentInfoForSendAndRecv {
  fn from(value: MsgContentInfo) -> Self {
    match value.content {
      MsgContent::MhyText(mhy_text) => Self {
        content: serde_json::to_value(mhy_text).unwrap(),
      },
      MsgContent::Unknown(MsgUnknown { content, .. }) => Self { content },
    }
  }
}
