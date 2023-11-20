/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

/// group info
#[derive(Debug, Deserialize)]
pub struct GroupInfo {
  /// id
  #[serde(
    rename = "group_id",
    deserialize_with = "deserialize_number_from_string"
  )]
  pub id: u64,
  /// name
  #[serde(rename = "group_name")]
  pub name: String,
}
