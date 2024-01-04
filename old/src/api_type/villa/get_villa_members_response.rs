/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

use crate::api_type::member::member_data::MemberData;

/// get villa members response
#[derive(Debug, Deserialize)]
pub struct GetVillaMembersResponse {
  /// list
  pub list: Vec<MemberData>,
  /// next offset str
  pub next_offset_str: String,
}
