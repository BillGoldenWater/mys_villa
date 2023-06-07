/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

/// create group request
#[derive(Debug, Serialize)]
pub struct CreateGroupRequest {
  group_name: String,
}

impl CreateGroupRequest {
  /// initialize with group_name
  pub fn new(group_name: String) -> Self {
    Self { group_name }
  }
}
