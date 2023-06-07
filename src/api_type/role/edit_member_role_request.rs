/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

use crate::api_type::role::role_color::RoleColor;
use crate::api_type::role::role_permission::RolePermission;

/// edit member role request
#[derive(Debug, Serialize)]
pub struct EditMemberRoleRequest {
  id: u64,
  name: String,
  color: RoleColor,
  permissions: Vec<RolePermission>,
}

impl EditMemberRoleRequest {
  /// initialize with id, name, color and permissions
  pub fn new(id: u64, name: String, color: RoleColor, permissions: Vec<RolePermission>) -> Self {
    Self {
      id,
      name,
      color,
      permissions,
    }
  }
}
