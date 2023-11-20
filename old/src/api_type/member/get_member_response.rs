/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

use crate::api_type::member::member_data::MemberData;

/// response of get member api
#[derive(Debug, Deserialize)]
pub struct GetMemberResponse {
  /// member data
  pub member: MemberData,
}
