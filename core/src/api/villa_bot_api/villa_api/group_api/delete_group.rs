/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DeleteGroup {
  pub group_id: u64,
}

impl DeleteGroup {
  pub fn new(group_id: u64) -> Self {
    Self { group_id }
  }
}
