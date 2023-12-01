/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::api::villa_bot_api::villa_api::send_msg_auth_range::SendMsgAuthRange;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Room {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub room_id: u64,
  pub room_name: String,
  pub room_type: String,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub group_id: u64,
  pub room_default_notify_type: String,
  pub send_msg_auth_range: SendMsgAuthRange,
}
