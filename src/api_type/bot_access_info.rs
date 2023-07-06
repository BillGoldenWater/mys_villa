/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::api_type::member::member_data::MemberData;

/// request definition of check bot access token of a member
pub mod check_member_bot_access_token_request;
/// response definition of check bot access token of a member
pub mod check_member_bot_access_token_response;

/// bot access info of a member
#[derive(Debug, Deserialize)]
pub struct BotAccessInfo {
  /// member uid
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub uid: u64,
  /// villa id
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub villa_id: u64,
  /// bot access token of a member
  #[serde(rename = "member_access_token")]
  pub bot_access_token: String,
  /// bot template id
  pub bot_tpl_id: String,
}

impl BotAccessInfo {
  /// initialize with uid, villa_id, bot_access_token and bot_tpl_id
  pub fn new(uid: u64, villa_id: u64, bot_access_token: String, bot_tpl_id: String) -> Self {
    Self { uid, villa_id, bot_access_token, bot_tpl_id }
  }
}

/// bot access data of a member
#[derive(Debug)]
pub struct BotAccessData {
  /// access info
  pub access_info: BotAccessInfo,
  /// member data
  pub member: MemberData,
}

impl BotAccessData {
  /// initialize with access_info and member
  pub fn new(access_info: BotAccessInfo, member: MemberData) -> Self {
    Self {
      access_info,
      member,
    }
  }
}
