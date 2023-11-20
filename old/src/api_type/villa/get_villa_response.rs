/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

use crate::api_type::villa::villa_info::VillaInfo;

/// get villa response
#[derive(Debug, Deserialize)]
pub struct GetVillaResponse {
  /// villa info
  pub villa: VillaInfo,
}
