/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

/// role type
#[derive(Debug, Deserialize)]
pub enum RoleType {
  /// MEMBER_ROLE_TYPE_ALL_MEMBER
  #[serde(rename = "MEMBER_ROLE_TYPE_ALL_MEMBER")]
  AllMember,
  /// MEMBER_ROLE_TYPE_ADMIN
  #[serde(rename = "MEMBER_ROLE_TYPE_ADMIN")]
  Admin,
  /// MEMBER_ROLE_TYPE_OWNER
  #[serde(rename = "MEMBER_ROLE_TYPE_OWNER")]
  Owner,
  /// MEMBER_ROLE_TYPE_CUSTOM
  #[serde(rename = "MEMBER_ROLE_TYPE_CUSTOM")]
  Custom,
  /// MEMBER_ROLE_TYPE_UNKNOWN
  #[serde(rename = "MEMBER_ROLE_TYPE_UNKNOWN")]
  Unknown,
}
