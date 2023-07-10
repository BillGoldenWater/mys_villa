/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

/// audit metadata
#[derive(Debug, Clone, Deserialize)]
pub struct AuditMetadata {
  /// audit id
  pub audit_id: String,
  /// bot template id
  pub bot_tpl_id: String,
  /// villa id
  pub villa_id: u64,
  /// room id
  pub room_id: Option<u64>,
  /// user id
  pub user_id: u64,
}
