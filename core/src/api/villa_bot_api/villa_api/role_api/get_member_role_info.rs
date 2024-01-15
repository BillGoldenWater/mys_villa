/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

use crate::api::villa_bot_api::villa_api::member_role::MemberRole;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMemberRoleInfoRequest {
  pub role_id: u64,
}

impl GetMemberRoleInfoRequest {
  pub fn new(role_id: u64) -> Self {
    Self { role_id }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMemberRoleInfoResponse {
  pub role: MemberRole,
}
