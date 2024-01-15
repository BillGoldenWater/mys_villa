/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EditMemberRoleRequest {
  pub id: u64,
  pub name: String,
  pub color: String,
  pub permissions: Vec<String>,
}

impl EditMemberRoleRequest {
  pub fn new(id: u64, name: String, color: String, permissions: Vec<String>) -> Self {
    Self {
      id,
      name,
      color,
      permissions,
    }
  }
}
