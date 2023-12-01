/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::impl_enum_str_convert;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RoleType {
  AllMember,
  Admin,
  Owner,
  Custom,
  Unknown(String),
}

impl_enum_str_convert! {
  RoleType;
  "MEMBER_ROLE_TYPE_ALL_MEMBER" => AllMember,
  "MEMBER_ROLE_TYPE_ADMIN" => Admin,
  "MEMBER_ROLE_TYPE_OWNER" => Owner,
  "MEMBER_ROLE_TYPE_CUSTOM" => Custom;
  _ => Unknown,
}
