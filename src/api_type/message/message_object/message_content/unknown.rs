/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Unknown {
  /// object name
  pub object_name: String,
  /// content
  pub content: Value,
}

impl Unknown {
  /// initialize with object name and content
  pub fn new(object_name: impl Into<String>, content: Value) -> Self {
    Self {
      object_name: object_name.into(),
      content,
    }
  }
}
