/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VillaInfo {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub villa_id: u64,
  pub name: String,
  pub villa_avatar_url: String,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub owner_uid: u64,
  pub is_official: bool,
  pub introduce: String,
  pub category_id: u32,
  pub tags: Vec<String>,
}
