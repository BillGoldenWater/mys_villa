/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

/// edit group request
#[derive(Debug, Serialize)]
pub struct EditGroupRequest {
  group_id: u64,
  group_name: String,
}

impl EditGroupRequest {
  /// initialize with group_id and group_name
  pub fn new(group_id: u64, group_name: String) -> Self {
    Self {
      group_id,
      group_name,
    }
  }
}
