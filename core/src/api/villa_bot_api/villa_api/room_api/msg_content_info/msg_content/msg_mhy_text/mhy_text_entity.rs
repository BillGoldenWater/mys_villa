/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

use crate::api::villa_bot_api::villa_api::room_api::msg_content_info::msg_content::msg_mhy_text::mhy_text_entity::entity_data::EntityData;

pub mod entity_data;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MhyTextEntity {
  pub entity: EntityData,
  pub length: u64,
  pub offset: u64,
}

impl MhyTextEntity {
  pub fn new(entity: EntityData, length: u64, offset: u64) -> Self {
    Self {
      entity,
      length,
      offset,
    }
  }
}
