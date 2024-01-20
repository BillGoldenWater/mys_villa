/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use crate::api::villa_bot_api::villa_api::room_api::msg_content_info::msg_content::msg_mhy_text::mhy_text_entity::MhyTextEntity;

pub mod mhy_text_entity;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsgMhyText {
  pub text: String,
  pub entities: Vec<MhyTextEntity>,
}

impl MsgMhyText {
  pub fn new(text: String, entities: Vec<MhyTextEntity>) -> Self {
    Self { text, entities }
  }
}
