/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

use crate::api::villa_bot_api::villa_api::member::Member;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GetMemberRequest {
  pub uid: u64,
}

impl GetMemberRequest {
  pub fn new(uid: u64) -> Self {
    Self { uid }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GetMemberResponse {
  pub member: Member,
}
