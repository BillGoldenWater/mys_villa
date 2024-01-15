/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::api::villa_bot_api::villa_api::list_room::ListRoom;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GroupRoom {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub group_id: u64,
  pub group_name: String,
  pub room_list: Vec<ListRoom>,
}
