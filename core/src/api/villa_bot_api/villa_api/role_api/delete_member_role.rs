/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMemberRoleRequest {
  pub id: u64,
}

impl DeleteMemberRoleRequest {
  pub fn new(id: u64) -> Self {
    Self { id }
  }
}
