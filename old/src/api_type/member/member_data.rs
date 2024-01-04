/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::{
  api_type::{member::member_info::MemberInfo, role::role_info::RoleInfo},
  utils::deserialize_number_vec_from_string_vec,
};

/// member data
#[derive(Debug, Clone, Deserialize)]
pub struct MemberData {
  /// member info
  #[serde(rename = "basic")]
  pub info: MemberInfo,
  /// join time, unix timestamp with second level precision
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub joined_at: u64,
  /// a list of role id of this member
  #[serde(deserialize_with = "deserialize_number_vec_from_string_vec")]
  pub role_id_list: Vec<u64>,
  /// a list of role info of this member
  pub role_list: Vec<RoleInfo>,
}
