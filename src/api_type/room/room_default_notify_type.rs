/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

/// room default notify type
#[derive(Debug, Deserialize)]
pub enum RoomDefaultNotifyType {
  /// BOT_PLATFORM_DEFAULT_NOTIFY_TYPE_NOTIFY
  #[serde(rename = "BOT_PLATFORM_DEFAULT_NOTIFY_TYPE_NOTIFY")]
  Notify,
  /// BOT_PLATFORM_DEFAULT_NOTIFY_TYPE_IGNORE  
  #[serde(rename = "BOT_PLATFORM_DEFAULT_NOTIFY_TYPE_IGNORE")]
  Ignore,
  /// BOT_PLATFORM_DEFAULT_NOTIFY_TYPE_INVALID  
  #[serde(rename = "BOT_PLATFORM_DEFAULT_NOTIFY_TYPE_INVALID")]
  Invalid,
}
