/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

use crate::api_type::role::role_color::RoleColor;
use crate::api_type::role::role_permission::RolePermission;

/// create member role request
#[derive(Debug, Serialize)]
pub struct CreateMemberRoleRequest {
  name: String,
  color: RoleColor,
  permissions: Vec<RolePermission>,
}

impl CreateMemberRoleRequest {
  /// initialize with name, color and permissions
  pub fn new(name: String, color: RoleColor, permissions: Vec<RolePermission>) -> Self {
    Self {
      name,
      color,
      permissions,
    }
  }
}
