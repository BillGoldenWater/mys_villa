/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

use crate::api_type::message::message_mhy_text::text_entity::TextEntity;

/// msg content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsgContent {
  /// text
  pub text: String,
  /// entities
  pub entities: Vec<TextEntity>,
}

impl MsgContent {
  /// initialize with text and entities
  pub fn new(text: impl Into<String>, entities: Vec<TextEntity>) -> Self {
    Self {
      text: text.into(),
      entities,
    }
  }
}
