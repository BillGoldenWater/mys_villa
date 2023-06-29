/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

/// entity data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EntityData {
  /// mentioned robot
  MentionedRobot {
    /// bot id
    bot_id: String,
  },
  /// mentioned user
  MentionedUser {
    /// user id
    user_id: String,
  },
  /// mentioned all
  MentionedAll,
  /// villa room link
  VillaRoomLink {
    /// villa id
    villa_id: String,
    /// room_id
    room_id: String,
  },
  /// link
  Link {
    /// url
    url: String,
    /// requires bot access token
    #[serde(default)]
    requires_bot_access_token: bool,
  },
}
