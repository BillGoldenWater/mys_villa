/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DeleteVillaMemberRequest {
  pub uid: u64,
}

impl DeleteVillaMemberRequest {
  pub fn new(uid: u64) -> Self {
    Self { uid }
  }
}
