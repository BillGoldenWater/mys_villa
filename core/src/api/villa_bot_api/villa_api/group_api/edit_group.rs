/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditGroupRequest {
  pub group_id: u64,
  pub group_name: String,
}

impl EditGroupRequest {
  pub fn new(group_id: u64, group_name: String) -> Self {
    Self {
      group_id,
      group_name,
    }
  }
}
