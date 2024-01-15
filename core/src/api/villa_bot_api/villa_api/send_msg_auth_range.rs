/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

use crate::utils::serde_utils::deserialize_number_vec_from_string_vec;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendMsgAuthRange {
  pub is_all_send_msg: bool,
  #[serde(deserialize_with = "deserialize_number_vec_from_string_vec")]
  pub roles: Vec<u64>,
}
