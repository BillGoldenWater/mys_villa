/*
 * Copyright 2023 - Golden_Water
 *
 * SPDX-License-Identifier: MIT
 */

use serde::Serialize;

/// audit request
#[derive(Debug, Serialize)]
pub struct AuditRequest {
  audit_content: String,
  pass_through: Option<String>,
  room_id: Option<u64>,
  uid: u64,
}

impl AuditRequest {
  /// initialize with audit_content, pass_through, room_id and uid
  pub fn new(
    audit_content: impl Into<String>,
    pass_through: Option<impl Into<String>>,
    room_id: Option<u64>,
    uid: u64,
  ) -> Self {
    Self {
      audit_content: audit_content.into(),
      pass_through: pass_through.map(Into::into),
      room_id,
      uid,
    }
  }
}
