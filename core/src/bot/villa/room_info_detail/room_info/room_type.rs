/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::impl_enum_str_convert;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RoomType {
  Chat,
  Live,
  Scene,
  Post,
  Invalid,
  Unknown(String),
}

impl_enum_str_convert! {
  RoomType;
  "BOT_PLATFORM_ROOM_TYPE_CHAT_ROOM" => Chat,
  "BOT_PLATFORM_ROOM_TYPE_LIVE_ROOM" => Live,
  "BOT_PLATFORM_ROOM_TYPE_SCENE_ROOM" => Scene,
  "BOT_PLATFORM_ROOM_TYPE_POST_ROOM" => Post,
  "BOT_PLATFORM_ROOM_TYPE_INVALID" => Invalid;
  _ => Unknown
}
