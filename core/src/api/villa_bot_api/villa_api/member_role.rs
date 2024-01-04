/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::{
  api::villa_bot_api::villa_api::member_role::member_role_permission::MemberRolePermission,
  utils::serde_utils::deserialize_number_vec_from_string_vec,
};

pub mod member_role_permission;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MemberRole {
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub id: u64,
  pub name: String,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub villa_id: u64,
  pub color: String,
  pub role_type: String,
  pub is_all_room: bool,
  #[serde(deserialize_with = "deserialize_number_vec_from_string_vec")]
  pub room_ids: Vec<u64>,
  #[serde(default)]
  pub permissions: Vec<MemberRolePermission>,
  // --- undocumented
  pub web_color: String,
  pub bg_color: String,
  pub color_scheme_id: i64,
  pub font_color: String,
  #[serde(default)]
  pub has_manage_perm: Option<bool>,
  #[serde(default, deserialize_with = "deserialize_option_number_from_string")]
  pub member_num: Option<u64>,
  pub priority: i64,
}
