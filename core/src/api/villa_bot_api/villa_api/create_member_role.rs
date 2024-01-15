/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateMemberRoleRequest {
  pub name: String,
  pub color: String,
  pub permissions: Vec<String>,
}

impl CreateMemberRoleRequest {
  pub fn new(name: String, color: String, permissions: Vec<String>) -> Self {
    Self {
      name,
      color,
      permissions,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateMemberRoleResponse {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub id: u64,
}
