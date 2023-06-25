/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::message::message_object::message_content::mhy_text::text_entity::TextEntity;
use serde::{Deserialize, Serialize};

/// definition of entity data
pub mod entity_data;
/// definition of text entity
pub mod text_entity;

/// mhy text
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MhyText {
  /// text
  pub text: String,
  /// entities
  pub entities: Vec<TextEntity>,
}

impl MhyText {
  /// initialize with text and entities
  pub fn new(text: impl Into<String>, entities: Vec<TextEntity>) -> Self {
    Self {
      text: text.into(),
      entities,
    }
  }
}
