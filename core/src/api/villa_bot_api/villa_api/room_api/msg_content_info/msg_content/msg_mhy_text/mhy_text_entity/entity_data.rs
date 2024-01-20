/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

use crate::api::villa_bot_api::villa_api::room_api::msg_content_info::msg_content::msg_mhy_text::mhy_text_entity::entity_data::font_style::FontStyle;

pub mod font_style;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EntityData {
  MentionedRobot {
    bot_id: String,
  },
  MentionedUser {
    user_id: String,
  },
  VillaRoomLink {
    villa_id: String,
    room_id: String,
  },
  Link {
    url: String,
    requires_bot_access_token: bool,
  },
  MentionAll,

  Style {
    font_style: FontStyle,
  },
}
