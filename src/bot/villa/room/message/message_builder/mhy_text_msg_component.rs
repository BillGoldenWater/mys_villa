/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

/// message component of mhy text
#[derive(Debug, Clone)]
pub enum MhyTextMsgComponent {
  /// text component
  Text(String),
  /// spacer component
  Spacer(String),
  /// mention user
  MentionUser {
    /// user name
    user_name: String,
    /// user id
    uid: u64,
  },
  /// mention all
  MentionAll,
  /// mention bot
  MentionBot {
    /// bot name
    bot_name: String,
    /// bot id
    bot_id: String,
  },
  /// villa room link
  VillaRoomLink {
    /// room name
    room_name: String,
    /// villa id
    villa_id: u64,
    /// room id
    room_id: u64,
  },
  /// link
  Link {
    /// link name
    link_name: String,
    /// link url
    url: String,
  },
}
