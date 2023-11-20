/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

/// create member role response
#[derive(Debug, Deserialize)]
pub struct CreateMemberRoleResponse {
  /// role id
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub id: u64,
}
