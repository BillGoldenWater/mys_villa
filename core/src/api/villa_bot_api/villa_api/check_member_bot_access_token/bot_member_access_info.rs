/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct BotMemberAccessInfo {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub uid: u64,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub villa_id: u64,

  pub bot_tpl_id: String,
  pub member_access_token: String,
}
