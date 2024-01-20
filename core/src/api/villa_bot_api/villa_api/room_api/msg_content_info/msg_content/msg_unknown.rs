/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MsgUnknown {
  pub object_name: Value,
  pub content: Value,
}

impl MsgUnknown {
  pub fn new(object_name: Value, content: Value) -> Self {
    Self {
      object_name,
      content,
    }
  }
}
