/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

use crate::api::villa_bot_api::villa_api::member::Member;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GetVillaMemberRequest {
  pub offset_str: String,
  pub size: u64,
}

impl GetVillaMemberRequest {
  pub fn new(offset_str: String, size: u64) -> Self {
    Self { offset_str, size }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GetVillaMemberResponse {
  pub list: Vec<Member>,
  pub next_offset_str: String,
}
