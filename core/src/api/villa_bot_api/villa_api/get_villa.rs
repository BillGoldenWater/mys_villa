/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::{Deserialize, Serialize};

use crate::api::villa_bot_api::villa_api::villa_info::VillaInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVillaResponse {
  pub villa: VillaInfo,
}
