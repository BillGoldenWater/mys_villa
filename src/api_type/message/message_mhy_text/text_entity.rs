use serde::{Deserialize, Serialize};

use crate::api_type::message::message_mhy_text::entity_data::EntityData;

/// text entity
#[derive(Debug, Clone, Serialize, Deserialize)]
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
