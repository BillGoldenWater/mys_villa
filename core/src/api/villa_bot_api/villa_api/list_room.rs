/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ListRoom {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub room_id: u64,
  pub room_name: String,
  pub room_type: String,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub group_id: u64,
}
