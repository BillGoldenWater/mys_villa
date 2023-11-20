/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

/// villa info
#[derive(Debug, Deserialize)]
pub struct VillaInfo {
  /// id
  #[serde(
    rename = "villa_id",
    deserialize_with = "deserialize_number_from_string"
  )]
  pub id: u64,
  /// name
  pub name: String,
  /// owner uid
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub owner_uid: u64,
  /// is official villa
  pub is_official: bool,
  /// avatar url
  #[serde(rename = "villa_avatar_url")]
  pub avatar_url: String,
  /// introduce
  pub introduce: String,
  /// category id
  pub category_id: u32,
  /// tags
  pub tags: Vec<String>,
}
