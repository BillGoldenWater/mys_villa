/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

use crate::api::villa_bot_api::villa_api::group::Group;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGroupListResponse {
  pub list: Vec<Group>,
}
