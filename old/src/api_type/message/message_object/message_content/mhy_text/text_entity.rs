/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::message::message_object::message_content::mhy_text::entity_data::EntityData;
use serde::{Deserialize, Serialize};

/// text entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextEntity {
  /// offset
  pub offset: u64,
  /// length
  pub length: u64,
  /// entity
  pub entity: EntityData,
}

impl TextEntity {
  /// initialize with offset, length and entity
  pub fn new(offset: u64, length: u64, entity: EntityData) -> Self {
    Self {
      offset,
      length,
      entity,
    }
  }
}
