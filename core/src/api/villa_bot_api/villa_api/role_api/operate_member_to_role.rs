/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperateMemberToRoleRequest {
  pub role_id: u64,
  pub uid: u64,
  pub is_add: bool,
}

impl OperateMemberToRoleRequest {
  pub fn new(role_id: u64, uid: u64, is_add: bool) -> Self {
    Self {
      role_id,
      uid,
      is_add,
    }
  }
}
