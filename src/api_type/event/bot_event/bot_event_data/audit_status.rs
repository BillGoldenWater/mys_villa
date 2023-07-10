/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Deserialize;

/// audit status
#[derive(Debug, Clone, Deserialize)]
#[serde(from = "i32")]
pub enum AuditStatus {
  /// for compatible
  Compatible,
  /// pass
  Passed,
  /// reject
  Rejected,
}

impl From<i32> for AuditStatus {
  fn from(value: i32) -> Self {
    match value {
      1 => Self::Passed,
      2 => Self::Rejected,
      _ => Self::Compatible,
    }
  }
}
