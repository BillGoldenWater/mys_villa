/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

/// create group response
#[derive(Debug, Deserialize)]
pub struct CreateGroupResponse {
  /// group id
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub group_id: u64,
}
