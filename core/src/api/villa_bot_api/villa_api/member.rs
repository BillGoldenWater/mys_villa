/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::api::villa_bot_api::villa_api::member::member_basic::MemberBasic;
use crate::api::villa_bot_api::villa_api::member_role::MemberRole;
use crate::utils::serde_utils::deserialize_number_vec_from_string_vec;

pub mod member_basic;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Member {
  pub basic: MemberBasic,
  #[serde(deserialize_with = "deserialize_number_vec_from_string_vec")]
  pub role_id_list: Vec<u64>,
  #[serde(deserialize_with = "deserialize_number_from_string")]
  pub joined_at: u64,
  pub role_list: Vec<MemberRole>,
}
