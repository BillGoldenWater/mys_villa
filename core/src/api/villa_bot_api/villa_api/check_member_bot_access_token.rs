/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

use bot_member_access_info::BotMemberAccessInfo;

use crate::api::villa_bot_api::villa_api::member::Member;

pub mod bot_member_access_info;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CheckMemberBotAccessTokenRequest {
  pub token: String,
}

impl CheckMemberBotAccessTokenRequest {
  pub fn new(token: String) -> Self {
    Self { token }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CheckMemberBotAccessTokenResponse {
  #[serde(default)]
  pub access_info: Option<BotMemberAccessInfo>,
  #[serde(default)]
  pub member: Option<Member>,
}
