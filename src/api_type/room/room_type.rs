/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

/// room type
#[derive(Debug, Deserialize)]
pub enum RoomType {
  /// BOT_PLATFORM_ROOM_TYPE_CHAT_ROOM
  #[serde(rename = "BOT_PLATFORM_ROOM_TYPE_CHAT_ROOM")]
  ChatRoom,
  /// BOT_PLATFORM_ROOM_TYPE_POST_ROOM
  #[serde(rename = "BOT_PLATFORM_ROOM_TYPE_POST_ROOM")]
  PostRoom,
  /// BOT_PLATFORM_ROOM_TYPE_SCENE_ROOM
  #[serde(rename = "BOT_PLATFORM_ROOM_TYPE_SCENE_ROOM")]
  SceneRoom,
  /// BOT_PLATFORM_ROOM_TYPE_INVALID
  #[serde(rename = "BOT_PLATFORM_ROOM_TYPE_INVALID")]
  Invalid,
}
