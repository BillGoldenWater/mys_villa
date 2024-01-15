/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemberBasic {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub uid: u64,
  pub nickname: String,
  pub introduce: String,
  #[serde(deserialize_with = "deserialize_option_number_from_string")]
  pub avatar: Option<u64>,
  pub avatar_url: String,
}
