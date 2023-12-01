/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

use crate::api::villa_bot_api::villa_api::group_room::GroupRoom;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct GetVillaGroupRoomListResponse {
  pub list: Vec<GroupRoom>,
}
