/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use crate::api_type::member::member_data::MemberData;
use serde::Deserialize;

/// get villa members response
#[derive(Debug, Deserialize)]
pub struct GetVillaMembersResponse {
  /// list
  pub list: Vec<MemberData>,
  /// next offset str
  pub next_offset_str: String,
}
