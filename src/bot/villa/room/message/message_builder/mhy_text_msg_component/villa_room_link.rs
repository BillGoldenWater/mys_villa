/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

/// villa room link
#[derive(Debug, Clone, PartialEq)]
pub struct VillaRoomLink {
  /// room name
  pub room_name: String,
  /// villa id
  pub villa_id: u64,
  /// room id
  pub room_id: u64,
}

impl VillaRoomLink {
  /// initialize with room name, villa id and room id
  pub fn new(room_name: impl Into<String>, villa_id: u64, room_id: u64) -> Self {
    Self {
      room_name: room_name.into(),
      villa_id,
      room_id,
    }
  }
}
