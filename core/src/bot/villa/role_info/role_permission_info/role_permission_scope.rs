/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::impl_enum_str_convert;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RolePermissionScope {
  Villa,
  Room,
  Unknown(String),
}

impl_enum_str_convert! {
  RolePermissionScope;
  "VILLA" => Villa,
  "ROOM" => Room;
  _ => Unknown
}
