/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

/// audit response
#[derive(Debug, Deserialize)]
pub struct AuditResponse {
  /// audit id
  pub audit_id: String,
}
