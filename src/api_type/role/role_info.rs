/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_aux::field_attributes::deserialize_option_number_from_string;

use crate::api_type::role::role_color::RoleColor;
use crate::api_type::role::role_permission_info::RolePermissionInfo;
use crate::api_type::role::role_type::RoleType;

/// role information
#[derive(Debug, Deserialize)]
pub struct RoleInfo {
  /// role id
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub id: u64,
  /// villa id
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub villa_id: u64,
  /// role name
  pub name: String,
  /// role type
  pub role_type: RoleType,
  /// permissions, will only return from Role::get_info
  #[serde(default)]
  pub permissions: Vec<RolePermissionInfo>,
  /// member number
  #[serde(deserialize_with = "deserialize_option_number_from_string", default)]
  pub member_num: Option<u64>,
  /// role color
  pub color: RoleColor,
  /// role web color
  pub web_color: String,
}
