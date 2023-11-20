/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

use crate::api_type::group::group_info::GroupInfo;

/// get group list response
#[derive(Debug, Deserialize)]
pub struct GetGroupListResponse {
  /// group list
  pub list: Vec<GroupInfo>,
}
