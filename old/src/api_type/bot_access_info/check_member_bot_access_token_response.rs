/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

use crate::api_type::{bot_access_info::BotAccessInfo, member::member_data::MemberData};

/// check member bot access token response
#[derive(Debug, Deserialize)]
pub struct CheckMemberBotAccessTokenResponse {
  /// bot access info
  #[serde(default)]
  pub access_info: Option<BotAccessInfo>,
  /// member data
  #[serde(default)]
  pub member: Option<MemberData>,
}
