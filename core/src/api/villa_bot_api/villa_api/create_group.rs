/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CreateGroupRequest {
  pub group_name: String,
}

impl CreateGroupRequest {
  pub fn new(group_name: String) -> Self {
    Self { group_name }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CreateGroupResponse {
  pub group_id: u64,
}
