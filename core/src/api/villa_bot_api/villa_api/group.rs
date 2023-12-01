/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Group {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub group_id: u64,
  pub group_name: String,
}
