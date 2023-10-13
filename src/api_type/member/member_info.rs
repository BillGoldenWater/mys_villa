/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;
use serde_aux::field_attributes::{
  deserialize_number_from_string, deserialize_option_number_from_string,
};

/// member information
#[derive(Debug, Clone, Deserialize)]
pub struct MemberInfo {
  /// avatar id
  #[serde(
    rename = "avatar",
    deserialize_with = "deserialize_option_number_from_string"
  )]
  pub avatar_id: Option<u64>,
  /// avatar url
  pub avatar_url: String,
  /// self description
  pub introduce: String,
  /// nick name
  pub nickname: String,
  /// member uid
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub uid: u64,
}
