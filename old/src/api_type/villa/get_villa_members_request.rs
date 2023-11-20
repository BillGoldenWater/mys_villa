/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

/// get villa members request
#[derive(Debug, Serialize)]
pub struct GetVillaMembersRequest {
  /// offset str
  offset_str: String,
  /// size
  size: u64,
}

impl GetVillaMembersRequest {
  /// initialize with offset_str and size
  pub fn new(offset_str: impl Into<String>, size: u64) -> Self {
    Self {
      offset_str: offset_str.into(),
      size,
    }
  }
}
