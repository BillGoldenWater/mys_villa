/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::{
  api::villa_bot_api::villa_api::member_role::member_role_permission::MemberRolePermission,
  bot::villa::role_info::role_permission_info::{
    role_permission_key::RolePermissionKey, role_permission_scope::RolePermissionScope,
  },
};

pub mod role_permission_key;
pub mod role_permission_scope;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RolePermissionInfo {
  pub key: RolePermissionKey,
  pub name: String,
  pub description: String,
  // --- undocumented
  pub scope: RolePermissionScope,
}

impl From<MemberRolePermission> for RolePermissionInfo {
  fn from(value: MemberRolePermission) -> Self {
    Self {
      key: value.key.into(),
      name: value.name,
      description: value.describe,
      scope: value.perm_scope.into(),
    }
  }
}

impl From<RolePermissionInfo> for MemberRolePermission {
  fn from(value: RolePermissionInfo) -> Self {
    Self {
      key: value.key.into(),
      name: value.name,
      describe: value.description,
      perm_scope: value.scope.into(),
    }
  }
}
