/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::impl_enum_str_convert;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RoomNotifyType {
  Notify,
  Ignore,
  Invalid,
  Unknown(String),
}

impl_enum_str_convert! {
  RoomNotifyType;
  "BOT_PLATFORM_DEFAULT_NOTIFY_TYPE_NOTIFY" => Notify,
  "BOT_PLATFORM_DEFAULT_NOTIFY_TYPE_IGNORE" => Ignore,
  "BOT_PLATFORM_DEFAULT_NOTIFY_TYPE_INVALID" => Invalid;
  _ => Unknown
}
